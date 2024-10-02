use std::cell::{Cell, RefCell};

use crate::compiler::types::{Arg, ByteCode, CompileError, Object, OpCode, RuntimeError};

static STACK_SIZE: usize = 10; //2048;

fn map_compile_err(err: CompileError) -> RuntimeError {
    RuntimeError(format!("{:?}", err))
}

pub struct VM {
    bytecode: ByteCode,
    stack: RefCell<Vec<Object>>,
    sp: Cell<usize>,
    ip: Cell<usize>,
    globals: RefCell<Vec<Object>>,

}

impl VM {
    pub fn new(bytecode: ByteCode) -> Self {
        let stack = vec![Object::Null; STACK_SIZE];
        Self {
            bytecode,
            stack: RefCell::new(stack),
            sp: Cell::new(0),
            ip: Cell::new(0),
            globals: RefCell::new(vec![Object::Null; STACK_SIZE]),
        }
    }

    pub fn run(&self) -> Result<(), RuntimeError> {
         loop {
            let mut ip = self.ip.get();
            // println!("IP: {}", ip);
            if ip >= self.bytecode.bytes.len() { break; }

            let opcode = OpCode::from_byte(self.bytecode.bytes[ip]).map_err(|err| map_compile_err(err))?;

            println!("Dbg: Executing opcode: {:?}", opcode);

            match opcode {
                OpCode::Constant => {
                    // let idx = match Arg::read_u16(&self.bytecode.bytes, ip) {
                    //     Ok(arg) => {
                    //         if let Arg::U16(x) = arg { x } else { unreachable!("Arg::read_u16 must return the Arg:U16 varient!"); }
                    //     },
                    //     Err(err) => return Err(map_compile_err(err))
                    // } as usize;
                    ip += 1;
                    let (_, idx) = Arg::read_u16(&self.bytecode.bytes, ip).map_err(map_compile_err)?;
                    let idx = idx as usize;
                    if idx >= self.bytecode.constants.len() {
                        return Err(RuntimeError(format!("Attempted to access object at index {}, but objects len is {}", idx, self.bytecode.constants.len())))
                    }

                    self.push_stack(self.bytecode.constants[idx].clone())?;

                    self.ip.set(ip + 2);
                },
                OpCode::Add => {
                    self.perform_infix_operation(|x, y| x + y, "+")?;
                },
                OpCode::Sub => {
                    self.perform_infix_operation(|x, y| x - y, "-")?;
                },
                OpCode::Mul => {
                    self.perform_infix_operation(|x, y| x * y, "*")?;
                },
                OpCode::Div => {
                    self.perform_infix_operation(|x, y| x / y, "/")?;
                },
                OpCode::Eq => {
                    self.perform_infix_operation(|x, y| Ok(Object::Boolean(x == y)), "==")?;
                },
                OpCode::NEq => {
                    self.perform_infix_operation(|x, y| Ok(Object::Boolean(x != y)), "!=")?;
                },
                OpCode::GT => {
                    self.perform_infix_operation(|x, y| Ok(Object::Boolean(x > y)), ">")?;
                },
                OpCode::LT => {
                    self.perform_infix_operation(|x, y| Ok(Object::Boolean(x < y)), "<")?;
                },
                OpCode::Minus => {
                    let val = self.pop_stack()?;
                    if let Object::Integer(val) = val {
                        self.push_stack(Object::Integer(-val))?;
                    } else {
                        return Err(RuntimeError(format!("`-` can only be applied to Integers, got: {val:?}")));
                    }

                    self.ip.set(ip + 1);
                },
                OpCode::Exclam => {
                    let val = self.pop_stack()?;
                    match val {
                        Object::Boolean(val) => self.push_stack(Object::Boolean(!val))?,
                        Object::Integer(val) => self.push_stack(Object::Boolean(val == 0))?,
                        Object::Null => self.push_stack(Object::Boolean(true))?,
                        _ => return Err(RuntimeError(format!("`!` can only be applied to Booleans and Integers got: {val:?}"))),
                    };

                    self.ip.set(ip + 1);
                }
                OpCode::Pop => {
                    self.pop_stack()?;

                    self.ip.set(ip + 1);
                },
                OpCode::True => {
                    self.push_stack(Object::Boolean(true))?;

                    self.ip.set(ip + 1);
                },
                OpCode::False => {
                    self.push_stack(Object::Boolean(false))?;

                    self.ip.set(ip + 1);
                },
                OpCode::Null => {
                    self.push_stack(Object::Null)?;

                    self.ip.set(ip + 1);
                },
                OpCode::JP => {
                    self.jump()?;
                },
                OpCode::JPTrue => {
                    let condition = self.pop_stack()?;
                    if condition.is_truthy() {
                        self.jump()?;
                    }else {
                        self.ip.set(ip + 3);
                    }
                },
                OpCode::JPFalse => {
                    let condition = self.pop_stack()?;
                    if !condition.is_truthy() {
                        self.jump()?;
                    }else {
                        self.ip.set(ip + 3);
                    }
                },
                OpCode::SetGlobal => {
                    let (_, idx) = Arg::read_u16(&self.bytecode.bytes, ip + 1).map_err(map_compile_err)?;
                    self.globals.borrow_mut()[idx as usize] = self.pop_stack()?;

                    self.ip.set(ip + 3);
                },
                OpCode::GetGlobal => {
                    let (_, idx) = Arg::read_u16(&self.bytecode.bytes, ip + 1).map_err(map_compile_err)?;
                    self.push_stack(self.globals.borrow()[idx as usize].clone())?;

                    self.ip.set(ip + 3);
                }
            }

            println!("Dbg: stack: {:?}", self.stack.borrow());
        }

        Ok(())
    }

    fn jump(&self) -> Result<(), RuntimeError> {
        // let addr = match Arg::read_u16(&self.bytecode.bytes, self.ip.get() + 1) {
        //     Ok(arg) => {
        //         if let Arg::U16(addr) = arg { addr } else { unreachable!("Arg::read_u16 must return the Arg:U16 varient!"); }
        //     },
        //     Err(err) => return Err(map_compile_err(err))
        // } as usize;
        let (_, addr) = Arg::read_u16(&self.bytecode.bytes, self.ip.get() + 1).map_err(map_compile_err)?;
        let addr = addr as usize;
        self.ip.set(addr);
        Ok(())
    }

    fn perform_infix_operation(&self, operator: fn(Object, Object) -> Result<Object, RuntimeError>, op_str: &str) -> Result<(), RuntimeError> {
        let y = self.pop_stack()?;
        let x = self.pop_stack()?;
        let res = operator(x.clone(), y.clone())?;
        println!("Dbg: {x:?} {op_str} {y:?} = {res:?}");
        self.push_stack(res)?;

        self.ip.set(self.ip.get() + 1);
        Ok(())
    }

    pub fn stack_top(&self) -> Result<Object, RuntimeError> {
        let sp = self.sp.get();
        if sp == 0 {
            Err(RuntimeError("stack_top: Cannot read empty stack!".to_string()))
        } else {
            Ok(self.stack.borrow()[sp - 1].clone())
        }
    }

    pub fn push_stack(&self, obj: Object) -> Result<(), RuntimeError> {
        let sp = self.sp.get();
        if sp == STACK_SIZE { return  Err(RuntimeError("push_stack: stack overflow".to_string())); }

        let mut stack = self.stack.borrow_mut();
        stack[sp] = obj;

        self.sp.set(sp + 1);
        Ok(())
    }

    pub fn pop_stack(&self) -> Result<Object, RuntimeError> {
        let val = self.stack_top()?;
        self.sp.set(self.sp.get() - 1);
        self.stack.borrow_mut()[self.sp.get()] = Object::Null;
        Ok(val)
    }
}

#[cfg(test)]
mod tests {

    use crate::{compiler::Compiler, lexer::Lexer, parser::Parser};

    use super::*;

    #[test]
    fn basic_test() {
        let test_case = "10 + 2 + 3 + 200";

        let lexer = Lexer::new(test_case.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        let mut compiler = Compiler::new();

        compiler.compile_program(&program).unwrap();
        let bytecode = compiler.get_byte_code();
        println!("bytecode: {:#?}", bytecode);

        let vm = VM::new(bytecode);

        vm.run().unwrap();

        println!("stack: {:#?}", vm.stack)

    }
}
