use std::ops::{Add, Div, Mul, Sub};

use crate::helpers::binary_helpers;

#[allow(dead_code)]
#[derive(Debug)]
pub struct CompileError(pub String);

#[allow(dead_code)]
#[derive(Debug)]
pub struct RuntimeError(pub String);

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

    pub fn read_u8(bytes: &Bytes, offset: usize) -> Result<Arg, CompileError> {
        if bytes.len() <= offset {
            return Err(CompileError("read_u8: offset larger than bytes size!".to_string()))
        }
        Ok(Arg::U8(bytes[offset]))
    }
    
    pub fn read_u16(bytes: &Bytes, offset: usize) -> Result<Arg, CompileError> {
        if bytes.len() <= offset + 1 {
            return Err(CompileError(format!("read_u16: offset: {} larger than bytes size: {}", offset, bytes.len())))
        }
        Ok(Arg::U16(binary_helpers::combine_bytes(bytes[offset], bytes[offset + 1])))
    }
}

// impl Add for Arg {
//     type Output = Self;
    
//     fn add(self, rhs: Self) -> Self::Output {
//         match (self, rhs) {
//             (Self::U8(x), Self::U8(y)) => Self::U8(x + y),
//             (Self::U16(x), Self::U16(y)) => Self::U16(x + y),
//             _ => panic!("Invalid addition: {:?} + {:?}", self, rhs),
//         }
//     }
// }

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Constant = 0,
    Pop = 1,
    Add = 2,
    Sub = 3,
    Mul = 4,
    Div = 5,
    True = 6,
    False = 7,
    Eq = 8,
    NEq = 9,
    GT = 10,
    Minus = 11,
    Exclam = 12,
}

impl OpCode {
    pub fn get_arg_widths(&self) -> Vec<u8> {
        match self {
            Self::Constant => vec![2],
            Self::Pop => Vec::new(),
            Self::Add => Vec::new(),
            Self::Sub => Vec::new(),
            Self::Mul => Vec::new(),
            Self::Div => Vec::new(),
            Self::True => Vec::new(),
            Self::False => Vec::new(),
            Self::Eq => Vec::new(),
            Self::NEq => Vec::new(),
            Self::GT => Vec::new(),
            Self::Minus => Vec::new(),
            Self::Exclam => Vec::new(),
        }
    }

    pub fn from_byte(opcode: u8) -> Result<Self, CompileError> {
        match opcode {
            _ if opcode == Self::Constant as u8 => Ok(Self::Constant),
            _ if opcode == Self::Pop as u8 => Ok(Self::Pop),
            _ if opcode == Self::Add as u8 => Ok(Self::Add),
            _ if opcode == Self::Sub as u8 => Ok(Self::Sub),
            _ if opcode == Self::Mul as u8 => Ok(Self::Mul),
            _ if opcode == Self::Div as u8 => Ok(Self::Div),
            _ if opcode == Self::True as u8 => Ok(Self::True),
            _ if opcode == Self::False as u8 => Ok(Self::False),
            _ if opcode == Self::Eq as u8 => Ok(Self::Eq),
            _ if opcode == Self::NEq as u8 => Ok(Self::NEq),
            _ if opcode == Self::GT as u8 => Ok(Self::GT),
            _ if opcode == Self::Minus as u8 => Ok(Self::Minus),
            _ if opcode == Self::Exclam as u8 => Ok(Self::Exclam),
            _ => Err(CompileError(format!("Unknown opcode: {opcode}")))
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

impl Add for Object {
    type Output = Result<Self, RuntimeError>;
    
    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Self::Integer(x), Self::Integer(y)) => Ok(Self::Integer(x + y)),
            _ => Err(RuntimeError(format!("Invalid addition: {:?} + {:?}", self, rhs))),
        }
    }
}

impl Sub for Object {
    type Output = Result<Self, RuntimeError>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Self::Integer(x), Self::Integer(y)) => Ok(Self::Integer(x - y)),
            _ => Err(RuntimeError(format!("Invalid subtraction: {:?} - {:?}", self, rhs))),
        }
    }
}

impl Mul for Object {
    type Output = Result<Self, RuntimeError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Self::Integer(x), Self::Integer(y)) => Ok(Self::Integer(x * y)),
            _ => Err(RuntimeError(format!("Invalid multiplication: {:?} * {:?}", self, rhs))),
        }
    }
}

impl Div for Object {
    type Output = Result<Self, RuntimeError>;

    fn div(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Self::Integer(x), Self::Integer(y)) => Ok(Self::Integer(x / y)),
            _ => Err(RuntimeError(format!("Invalid division: {:?} / {:?}", self, rhs))),
        }
    }
}

pub type Constants = Vec<Object>;
#[derive(Debug)]
pub struct ByteCode {
    pub bytes: Bytes,
    pub constants: Constants
}
