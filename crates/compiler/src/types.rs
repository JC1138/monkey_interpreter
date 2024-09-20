#[allow(dead_code)]
#[derive(Debug)]
pub struct CompileError(pub String);

pub type Bytes = Vec<u8>;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arg {
    U8(u8),
    U16(u16)
}

impl Arg {
    pub fn get_size(&self) -> u8 {
        match self {
            Self::U8(_) => 8,
            Self::U16(_) => 16,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Constant = 0
}

impl OpCode {
    pub fn get_arg_widths(&self) -> Vec<u8> {
        match self {
            Self::Constant => vec![2],
        }
    }

    pub fn from_byte(opcode: u8) -> Result<Self, CompileError> {
        match opcode {
            _ if opcode == Self::Constant as u8 => Ok(Self::Constant),
            _ => Err(CompileError(format!("Unknown opcode: {opcode}")))
        }
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Integer(isize),
    Boolean(bool),
    String(String),
    Array(Vec<Self>),
    KVPair(Box<Self>, Box<Self>),
    Return(Box<Self>),
    Null,

    BuiltIn(fn(Vec<Object>) -> Result<Object, CompileError>)
}


pub type Constants = Vec<Object>;
#[derive(Debug)]
pub struct ByteCode {
    pub bytes: Bytes,
    pub constants: Constants
}
