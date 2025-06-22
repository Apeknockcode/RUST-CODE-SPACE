use std::fmt::{Display, Formatter};

use rust_decimal::Decimal;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum  Token {
    Add, //
    Substract,
    Multiply,
    Divide,
    Caret,
    LeftParen, // 左括号
    RightParen, //右括号
    Number(Decimal), //
    EOF, // 正常结束
}

// 给token 定义方法
// TODO：（1）计算的优先级
impl  Token  {
    pub fn get_precedence(&self) -> OperatorPrecedence {
        // 局部导入
        use Token::*;
        use OperatorPrecedence::*;
        match self {
            Add | Substract => AddOrSubstract,
            Multiply | Divide => MultiplyOrDivide,
            Caret =>Power,
            _ => Default,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        match  self {
            Add => write!(f, "+"),
            Substract => write!(f, "-"),
            Multiply => write!(f, "*"),
            Divide => write!(f, "/"),
            Caret => write!(f, "^"),
            LeftParen => write!(f, "("),
            RightParen => write!(f, ")"),
            Number(n) => write!(f, "{}", n),
            EOF => write!(f, "EOF"),
        }
    }
}


// TODO: (1-1) 定义计算的优先级结构
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy )]
// 优先级：Default > AddOrSubtract> MultiplyOrDivide > Power> Negative
pub enum  OperatorPrecedence {
    Default, // 默认优先级
    AddOrSubstract, // 加减优先级
    MultiplyOrDivide, // 乘除优先级
    Power, // 幂运算优先级
    Negative, // 取反优先级
}