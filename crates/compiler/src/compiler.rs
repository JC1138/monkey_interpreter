use crate::helpers::{self, binary_helpers};

pub use crate::types::*;

use parser::{ast::{self, Statement}, Program};

pub fn unmake(bytes: &Bytes, offset: usize) -> Result<(OpCode, Vec<Arg>, usize), CompileError> {
    if bytes.len() <= offset {
        return Err(CompileError("unmake: offset larger than bytes size!".to_string()))
    }
    let opcode = OpCode::from_byte(bytes[offset])?;

    let mut bytes_read: usize = 1;

    let arg_widths = opcode.get_arg_widths();
    let total_len = arg_widths.iter().sum::<u8>() + 1;

    // println!("total_len: {total_len}, offset: {offset}, len: {}", bytes.len());
    if total_len as usize + offset - 1 >= bytes.len() {
        return Err(CompileError("unmake: args length greater than bytes size!".to_string()))
    }

    let mut args = Vec::new();
    for width in arg_widths {
        match width {
            1 => args.push(Arg::read_u8(bytes, offset + bytes_read)?),
            2 => args.push(Arg::read_u16(bytes, offset + bytes_read)?),
            _ => return  Err(CompileError(format!("Invalid arg width: {}", width))),
        }
        bytes_read += width as usize;
    }

    Ok((opcode, args, bytes_read))
}

pub fn make(opcode: OpCode, args: &Vec<Arg>) -> Result<Vec<u8>, CompileError> {
    let arg_widths = opcode.get_arg_widths();

    if args.len() != arg_widths.len() {
        return Err(CompileError(format!("Cannot compile opcode: {opcode:?}, expected {} args, got: {}", arg_widths.len(), args.len())))
    }

    let mut bytes = vec![opcode as u8];

    for i in 0..args.len() {
        let arg = args[i];
        let width = arg_widths[i];

        match (width, arg) {
            (1, Arg::U8(val)) => bytes.push(val),
            (2, Arg::U16(val)) => {
                let (h, l) = binary_helpers::split_u16(val);
                bytes.extend_from_slice(&[h, l]);
            },
            _ => return Err(CompileError(format!("Cannot parse arg: {:?} for opcode: {:?}, expected size: {} byte(s)", arg, opcode, width))),
        }
    }

    let expected_len = arg_widths.iter().sum::<u8>() + 1; // +1 for opcode
    if expected_len as usize != bytes.len() {
        return Err(CompileError(format!("Invalid bytecode, expected {}, got: {}", expected_len, bytes.len())))
    }

    Ok(bytes)
}

pub struct Compiler {
    bytes: Bytes,
    constants: Constants
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            constants: Vec::new(),
        }
    }

    fn add_constant(&mut self, obj: Object) -> usize {
        self.constants.push(obj);
        self.constants.len() - 1
    }

    fn emit(&mut self, opcode: OpCode, args: &Vec<Arg>) -> Result<usize, CompileError> {
        let bytes = make(opcode, args)?;
        let start = self.bytes.len();
        self.bytes.extend(bytes);
        Ok(start)
    }

    fn emit_no_args(&mut self, opcode: OpCode) -> Result<usize, CompileError> {
        self.emit(opcode, &Vec::new())
    }

    pub fn compile_program(&mut self, program: &Program) -> Result<ByteCode, CompileError> {
        for statement in &program.statements {
            self.compile_statement(statement)?;
        }
        Ok(self.get_byte_code())
    }

    fn parse_statements(&mut self, statements: &Vec<Statement>) -> Result<(), CompileError> {
        for statement in statements {
            self.compile_statement(statement)?;
        }
        Ok(())
    }

    fn compile_statement(&mut self, statement: &ast::Statement) -> Result<(), CompileError> {
        match statement {
            ast::Statement::ExpressionStatement { expression, .. } => {
                self.compile_expression(expression)?;
                self.emit(OpCode::Pop, &Vec::new())?;
            },
            ast::Statement::Block { statements, .. } => {
                for statement in statements {
                    self.compile_statement(statement)?;
                }
            }
            // ast::Statement::Block { statements, .. } => self.
            _ => return Err(CompileError(format!("Compilation not implemented for: {:?}", statement))),
        }
        
        Ok(())
    }

    fn compile_expression(&mut self, expression: &ast::Expression) -> Result<(), CompileError> {
        match expression {
            ast::Expression::Infix { left, operator, right, .. } => {
                self.compile_expression(left)?;
                self.compile_expression(right)?;
                match operator.as_str() {
                    "+" => { self.emit_no_args(OpCode::Add)?; },
                    "-" => { self.emit_no_args(OpCode::Sub)?; },
                    "*" => { self.emit_no_args(OpCode::Mul)?; },
                    "/" => { self.emit_no_args(OpCode::Div)?; },
                    "==" => { self.emit_no_args(OpCode::Eq)?; },
                    "!=" => { self.emit_no_args(OpCode::NEq)?; },
                    ">" => { self.emit_no_args(OpCode::GT)?; },
                    "<" => { self.emit_no_args(OpCode::LT)?; },
                    op @ _ => return Err(CompileError(format!("Cannot compile infix operator: {}", op))),
                }
            },
            ast::Expression::Integer { value, .. } => {
                let idx = self.add_constant(Object::Integer(*value));
                self.emit(OpCode::Constant, &vec![Arg::U16(idx as u16)])?;
            },
            ast::Expression::Boolean { value, .. } => {
                let opcode = if *value { OpCode::True } else { OpCode::False };
                self.emit(opcode, &Vec::new())?;
            },
            ast::Expression::Prefix { operator, right, .. } => {
                self.compile_expression(&right)?;
                
                match operator.as_str() {
                    "-" => { self.emit_no_args(OpCode::Minus)?; },
                    "!" => { self.emit_no_args(OpCode::Exclam)?; },
                    op @ _ => return Err(CompileError(format!("Cannot compile prefix operator: {}", op))),
                }
            },
            ast::Expression::If { condition, consequence, alternative, .. } => {
                self.compile_expression(&condition)?;
                self.emit(OpCode::JPFalse, &vec![Arg::U16(0)])?;

                let jp_false_addr_idx = self.bytes.len() - 2;

                self.compile_statement(&consequence)?;
                self.remove_last_pop();

                let mut jp_false_addr = self.bytes.len();

                if let Some(alternative) = alternative {
                    self.emit(OpCode::JP, &vec![Arg::U16(0)])?;
                    jp_false_addr = self.bytes.len();

                    let jp_addr_idx = self.bytes.len() - 2;

                    self.compile_statement(&alternative)?;
                    self.remove_last_pop();

                    let jp_addr = self.bytes.len();

                    self.overwrite_address(jp_addr_idx, jp_addr as u16);
                }

                self.overwrite_address(jp_false_addr_idx, jp_false_addr as u16);
            }
            _ => return Err(CompileError(format!("Compilation not implemented for: {:?}", expression))),
        }
        Ok(())
    }

    fn remove_last_pop(&mut self) {
        if let Some(val) = self.bytes.last() {
            if *val == OpCode::Pop as u8 {
                self.bytes.pop();
            }
        }
    }

    fn overwrite_address(&mut self, addr_idx: usize, addr: u16) {
        let (h, l) = binary_helpers::split_u16(addr);
        self.bytes[addr_idx] = h;
        self.bytes[addr_idx + 1] = l;
    }

    pub fn get_byte_code(&self) -> ByteCode {
        ByteCode {
            bytes: self.bytes.clone(),
            constants: self.constants.clone(),
        }
    }

    pub fn reset(&mut self) {
        self.bytes.clear();
        self.constants.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_make_constant() -> Result<(), CompileError> {
        assert_eq!(make(OpCode::Constant, &vec![Arg::U16(0xfffe)])?, vec![OpCode::Constant as u8, 0xff, 0xfe]);

        Ok(())
    }

    #[test]
    fn test_unmake_constant() -> Result<(), CompileError> {
        assert_eq!(unmake(&vec![0, 0xab, 0xcd], 0)?, (OpCode::Constant, vec![Arg::U16(0xabcd)], 3));
        Ok(())
    }
}
