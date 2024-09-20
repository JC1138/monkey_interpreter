use std::{cell::RefCell, collections::HashMap, hash::{DefaultHasher, Hash, Hasher}, rc::{Rc, Weak}};

use parser::{ast::{self, Expression, Statement}, Program};

#[allow(dead_code)]
#[derive(Debug)]
pub struct EvalError(String);

#[derive(Debug, Clone)]
pub enum Object {
    Integer(isize),
    Boolean(bool),
    String(String),
    Array(Vec<Self>),
    KVPair(Box<Self>, Box<Self>),
    HashMap(HashMap<HashKey, Self>),
    Return(Box<Self>),
    Function {
        parameters: Vec<String>, // Identifiers
        body: ast::Statement,    // Block statement
        fn_env: Weak<RefCell<Environment>>,
    },
    Null,

    BuiltIn(fn(Vec<Object>) -> Result<Object, EvalError>)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HashKey {
    pub typ: String,
    pub value: usize,
}

impl HashKey {
    pub fn get_hash_key(object: &Object) -> Result<Self, EvalError> {
        match object {
            Object::Integer(value) => Ok(Self { typ: "int".to_string(), value: *value as usize}),
            Object::Boolean(value) => Ok(Self { typ: "bool".to_string(), value: if *value {1} else {0}}),
            Object::String(value) => {
                let mut hasher = DefaultHasher::new();
                value.hash(&mut hasher);
                Ok(Self { typ: "str".to_string(), value: hasher.finish() as usize})
            },
            _ => Err(EvalError(format!("Cannot hash object: {object:?}"))),
        }
    }
}

impl Object {
    pub fn construct_fn(parameters: &Vec<ast::Expression>, body: &ast::Statement, env: &Env) -> Result<Object, EvalError> {
        let mut param_names: Vec<String> = Vec::new();
        if matches!(body, ast::Statement::Block { .. }) {
            for param in parameters {
                if let ast::Expression::Identifier { value, .. } = param {
                    param_names.push(value.to_string());
                } else {
                    return Err(EvalError(format!("Invalid fn parameters: {parameters:?}, all parameters must be Identifiers, got: {param:?}")));
                }
            }
            Ok(Self::Function { parameters: param_names, body: body.clone(), fn_env: Rc::downgrade(&env) })
        } else {
            return Err(EvalError(format!("Invalid fn body: {body:?}, must be Block statemnt")))
        }
    }

    pub fn unwrap_return(self) -> Self {
        if let Self::Return(return_val) = self {
            return return_val.unwrap_return()
        }
        self
    }

    // pub fn unwrap_kv_pair(self) -> Self {
    //     if let Self::KVPair(, )
    // }
}

pub type Env = Rc<RefCell<Environment>>;
#[derive(Debug)]
pub struct Environment {
    vars: HashMap<String, Object>,
    outer: Option<Env>
}

impl Environment {
    pub fn new(outer: Option<Env>) -> Self {
        Self {
            vars: HashMap::new(),
            outer,
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        if let Some(obj) = self.vars.get(name) {
            return Some(obj.clone());
        }

        if let Some(outer_env) = &self.outer {
            return outer_env.borrow().get(name);
        }

        None
    }

    pub fn set(&mut self, name: &str, val: Object) {
        self.vars.insert(name.to_string(), val);
    }
}

pub struct Interpreter {
    envs: RefCell<Vec<Env>>,
}

impl Interpreter {
    pub fn new(mut global_env: Environment) -> Self {
        fn check_num_args(args: &Vec<Object>, num_args: usize) -> Result<(), EvalError> {
            if args.len() != num_args {  Err(EvalError(format!("Error in built-in len, expected 1 arguement, got: {}", args.len()))) } else { Ok(()) }
        }
        global_env.set("len", Object::BuiltIn(|args| {
            check_num_args(&args, 1)?;
            match &args[0] {
                Object::String(str) => Ok(Object::Integer(str.len() as isize)),
                Object::Array(arr) => Ok(Object::Integer(arr.len() as isize)),
                _ => Err(EvalError(format!("Can't call built-in fn `len` on type: {:?}", args[0])))
            }
        }));

        global_env.set("first", Object::BuiltIn(|args| {
            check_num_args(&args, 1)?;
            match &args[0] {
                Object::Array(arr) => Ok( if arr.len() > 0 { arr[0].clone() } else { Object::Null }),
                _ => Err(EvalError(format!("Can't call built-in fn `first` on type: {:?}", args[0])))
            }
        }));

        global_env.set("last", Object::BuiltIn(|args| {
            check_num_args(&args, 1)?;
            match &args[0] {
                Object::Array(arr) => Ok( if arr.len() > 0 { arr[arr.len() - 1].clone() } else { Object::Null }),
                _ => Err(EvalError(format!("Can't call built-in fn `last` on type: {:?}", args[0])))
            }
        }));

        global_env.set("rest", Object::BuiltIn(|args| {
            check_num_args(&args, 1)?;
            match &args[0] {
                Object::Array(arr) => 
                    Ok( if arr.len() > 0 { 
                        let mut arr = arr.clone(); 
                        arr.remove(0); 
                        Object::Array(arr) 
                    } else { Object::Null }),
                _ => Err(EvalError(format!("Can't call built-in fn `rest` on type: {:?}", args[0])))
            }
        }));

        global_env.set("push", Object::BuiltIn(|args| {
            check_num_args(&args, 2)?;
            match (&args[0], &args[1]) {
                (Object::Array(arr), val @ _) => {
                    let mut arr = arr.clone();
                    arr.push(val.clone());
                    Ok(Object::Array(arr))
                }
                _ => Err(EvalError(format!("Can't call built-in fn `push` on type: {:?}", args[0])))
            }
        }));

        global_env.set("print", Object::BuiltIn(|args| {
            check_num_args(&args, 1)?;
            match &args[0] {
                Object::String(str) => Ok(Object::String(str.to_string())),
                _ => Err(EvalError(format!("Can't call built-in fn `print` on type: {:?}", args[0])))
            }
        }));

        global_env.set("println", Object::BuiltIn(|args| {
            check_num_args(&args, 1)?;
            match &args[0] {
                Object::String(val) => println!("{}", val),
                Object::Integer(val) => println!("{}", val),
                Object::Boolean(val) => println!("{}", val),
                _ => return Err(EvalError(format!("Can't call built-in fn `println` on type: {:?}", args[0])))
            };
            Ok(args[0].clone())
        }));

        Self {
            envs: RefCell::new(vec![Rc::new(RefCell::new(global_env))]),
        }
    }

    pub fn evaluate_program(&self, program: &Program) -> Result<Object, EvalError> {
        let first_env = Rc::clone(&self.envs.borrow()[0]);
        self.eval_statements(&program.statements, false, &first_env)
    }
    
    fn eval_statements(&self, statements: &Vec<Statement>, is_block: bool, env: &Env) -> Result<Object, EvalError> {
    
        let mut result = Object::Null;
        for statement in statements {
            result = self.eval_statement(statement, env)?;
            if let Object::Return(_) = result {
                if is_block {
                    return Ok(result) // if in a block statement, we don't want to unwrap the return value
                }
                return Ok(result.unwrap_return());
            }
        }
    
        Ok(result)
    }
    
    fn eval_statement(&self, statement: &Statement, env: &Env) -> Result<Object, EvalError> {
        match statement {
            Statement::ExpressionStatement { expression, .. } => self.eval_expression(&expression, env),
            Statement::Block { statements, .. } => self.eval_statements(statements, true, env),
            Statement::Return { return_value, .. } => self.eval_return_statement(&return_value, env),
            Statement::Let { name, value, .. } => self.eval_let_statement(name, value, env),
        }
    }
    
    fn eval_return_statement(&self, return_value: &ast::Expression, env: &Env) -> Result<Object, EvalError> {
        let return_value = self.eval_expression(return_value, env)?;
        Ok(Object::Return(Box::new(return_value)))
    }
    
    fn eval_let_statement(&self, name: &ast::Expression, value: &ast::Expression, env: &Env) -> Result<Object, EvalError> {
        let val = self.eval_expression(value, env)?;
        if let ast::Expression::Identifier { value, .. } = name {
            env.borrow_mut().set(value, val.clone());
            Ok(val)
        } else {
            Err(EvalError(format!("Invalid let statement, expected identifier, got: {name:?}")))
        }
    }
    
    fn eval_expression(&self, expression: &ast::Expression, env: &Env) -> Result<Object, EvalError> {
        match expression {
            ast::Expression::Integer { value, .. } => Ok(Object::Integer(*value)),
            ast::Expression::Boolean { value, .. } => Ok(Object::Boolean(*value)),
            ast::Expression::String { value, .. } => Ok(Object::String(value.to_string())),
            ast::Expression::Array { elements, .. } => {
                let eval_elms = elements
                    .iter()
                    .map(|exp| self.eval_expression(exp, env)).collect::<Result<Vec<Object>, EvalError>>()?;
               Ok(Object::Array(eval_elms))
            },
            ast::Expression::KVPair { key, value } => {
                let key = self.eval_expression(key, env)?;
                match key {
                    Object::String(_) | Object::Integer(_) | Object::Boolean(_) => Ok(Object::KVPair(Box::new(key), Box::new(self.eval_expression(value, env)?))),
                    _ => Err(EvalError(format!("Invalid KV pair, key must be a string, int or bool, got: {key:?}")))
                }
            },
            ast::Expression::Hash { kv_pairs } => {
                let mut hash_map = HashMap::new();
                for kv_pair in kv_pairs {
                    if let ref kv_pair @ Object::KVPair(ref key, ..) = self.eval_expression(kv_pair, env)? {
                        hash_map.insert(HashKey::get_hash_key(&key)?, kv_pair.clone());
                    } else {
                        return Err(EvalError(format!("Invalid hash map, all entries must be a kv pair, got: {kv_pair:?}")));
                    }
                }

                Ok(Object::HashMap(hash_map))
            },
            ast::Expression::Index { name, i, .. } => {
                let i = self.eval_expression(i, env)?;
                match self.eval_expression(name, env)? {
                    Object::Array(arr) => {
                        if let Object::Integer(index) = i {
                            let index = index as usize;
                            if index >= arr.len() {
                                return Err(EvalError(format!("Array index out of bounds: i: {}, {}.len(): {}", index, name.as_ref().dbg(),  arr.len())))
                            } else {
                                return Ok(arr[index].clone())
                            }
                        } else {
                            return Err(EvalError(format!("Invalid array index expression, expected int, got: {i:?}")))
                        }
                    },
                    Object::HashMap(hash_map) => {
                        let hash_key = HashKey::get_hash_key(&i)?;
                        if let Some(kv_pair) = hash_map.get(&hash_key) {
                            if let Object::KVPair(_, value) = kv_pair {
                                Ok(*value.clone())
                            } else {
                                Ok(Object::Null)
                            }
                        } else {
                            Ok(Object::Null)
                        }
                    }
                    _ => Err(EvalError(format!("Invalid array index expression: ({:?})[{:?}]", name, i )))
                }
            }
            ast::Expression::Prefix { operator, right, .. } => {
                let right = self.eval_expression(right, env)?;
                self.eval_prefix_expression(operator, right)
            },
            ast::Expression::Infix { left, operator, right, .. } => {
                let left = self.eval_expression(left, env)?;
                let right = self.eval_expression(right, env)?;
                self.eval_infix_expression(left, operator, right)
            },
            ast::Expression::If { condition, consequence, alternative, .. } => {
                let condition = self.eval_expression(condition, env)?;
                self.eval_if_expression(condition, consequence, alternative, env)
            },
            ast::Expression::Identifier { value, .. } => env.borrow().get(value).ok_or(EvalError(format!("Unknown variable: {value}"))),
            ast::Expression::Function { params, body, .. } => {
                let cur_env = Rc::clone(&env);
                self.envs.borrow_mut().push(cur_env);
                Object::construct_fn(params, body, env)
            },
            ast::Expression::Call { function, arguements, .. } => self.eval_call_expression(function, arguements, env),
            // _ => Err(EvalError("".to_string()))
        }
    }
    
    fn eval_prefix_expression(&self, operator: &str, right: Object) -> Result<Object, EvalError> {
        match operator {
            "!" => {
                match right {
                    Object::Integer(val) => Ok(Object::Boolean(val == 0)),
                    Object::Boolean(val) => Ok(Object::Boolean(!val)),
                    Object::Null => Ok(Object::Boolean(true)),
                    _ => Err(EvalError(format!("Invalid arg {right:?} for prefix operator {operator}")))
                }
            },
            "-" => {
                match right {
                    Object::Integer(val) => Ok(Object::Integer(-val)),
                    _ => Err(EvalError(format!("Invalid arg {right:?} for prefix operator {operator}")))
                }
            },
            _ => Err(EvalError(format!("Cannot eval prefix expression: {operator}{right:?}"))),
        }
    }
    
    fn eval_infix_expression(&self, left: Object, operator: &str, right: Object) -> Result<Object, EvalError> {
        let left = left.unwrap_return();
        let right: Object = right.unwrap_return();

        match (&left, &right) {
            (Object::Integer(left_val), Object::Integer(right_val)) => {
                Ok(match operator {
                    "+" => Object::Integer(left_val + right_val),
                    "-" => Object::Integer(left_val - right_val),
                    "*" => Object::Integer(left_val * right_val),
                    "/" => Object::Integer(left_val / right_val),
                    ">" => Object::Boolean(left_val > right_val),
                    "<" => Object::Boolean(left_val < right_val),
                    "==" => Object::Boolean(left_val == right_val),
                    "!=" => Object::Boolean(left_val != right_val),
                    _ => return Err(EvalError(format!("Invalid operator in infix position: {left:?}{operator}{right:?}"))),
                })
            },
            (Object::Boolean(left_val), Object::Boolean(right_val)) => {
                Ok(match operator {
                    ">" => Object::Boolean(left_val > right_val),
                    "<" => Object::Boolean(left_val < right_val),
                    "==" => Object::Boolean(left_val == right_val),
                    "!=" => Object::Boolean(left_val != right_val),
                    _ => return Err(EvalError(format!("Invalid operator in infix position: {left:?}{operator}{right:?}"))),
                })
            },
            (Object::String(left_val), Object::String(right_val)) => {
                Ok(match operator {
                    "+" => Object::String(left_val.to_string() + right_val),
                    "==" => Object::Boolean(left_val == right_val),
                    "!=" => Object::Boolean(left_val != right_val),
                    _ => return Err(EvalError(format!("Invalid operator in infix position: {left:?}{operator}{right:?}"))),
                })
            },

            _ => Err(EvalError(format!("Type mismatch {left:?} {operator} {right:?}")))
        }
    }
    
    fn eval_if_expression(&self, condition: Object, consequence: &Box<Statement>, alternative: &Option<Box<Statement>>, env: &Env) -> Result<Object, EvalError> {
        let mut bool_condition = false;
        if let Object::Integer(val) = condition {
            bool_condition = val != 0;
        }
    
        if let Object::Boolean(val) = condition {
            bool_condition = val;
        }
    
        if bool_condition {
            match consequence.as_ref() {
                Statement::Block { statements, .. } => self.eval_statements(&statements, true, env),
                _ => Err(EvalError(format!("Consequence must be a block statement, got: {consequence:?}")))
            }
        } else {
            if let Some(alt) = alternative {
                match alt.as_ref() {
                    Statement::Block { statements, .. } => self.eval_statements(&statements, true, env),
                    _ => Err(EvalError(format!("Alternative must be a block statement, got: {alt:?}")))
                }
            }else {
                Ok(Object::Null)
            }
        }
    }
    
    fn eval_call_expression(&self, function: &Box<Expression>, arguements: &Vec<Expression>, env: &Env) -> Result<Object, EvalError> {
        let function_obj = &self.eval_expression(function, env)?.unwrap_return();
    
        if let Object::Function { parameters, body, fn_env } = function_obj {
            if parameters.len() != arguements.len() {
                return Err(EvalError(format!("Invalid call expression, expected {:?} args, got: {:?}, function obj: {:?}", parameters.len(), arguements.len(), function_obj)));
            }
    
            if let ast::Statement::Block { statements, .. } = body {
                let new_env = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&fn_env.upgrade().expect(&format!("Unable to get fn_env!: function: {function:?}, function_obj: {function_obj:?}")))))));
    
                for i in 0..arguements.len() {
                    new_env.borrow_mut().set(&parameters[i], self.eval_expression(&arguements[i], env)?)
                }
    
                return Ok(self.eval_statements(statements, true, &Rc::clone(&new_env))?.unwrap_return())
            } else {
                return Err(EvalError(format!("Invalid call expression, function body: {body:?} must be Block statement")))
            }
        }

        if let Object::BuiltIn(f) = function_obj {
            let mut args = Vec::new();
            for i in 0..arguements.len() {
                args.push(self.eval_expression(&arguements[i], env)?)
            }
            return f(args)
        } 
    
        Err(EvalError(format!("Invalid call expression, expression: {function:?} must evalate to function, got: {function_obj:?}")))
    }
    
}
