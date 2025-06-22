use crate::calc::ast::Node;
use crate::calc::error::{CalcError, CalcResult};
use crate::calc::token::OperatorPrecedence;
use crate::calc::token::Token;
// TODO : 定义结构题
use crate::calc::tokenizer::Tokenizer;
pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(expression: &'a str) -> CalcResult<Self> {
        let mut tokenizer = Tokenizer::new(expression);
        let current_token = tokenizer
            .next()
            .ok_or_else(|| CalcError::UnexpectedChar(tokenizer.get_unexpected_char().unwrap()))?;
        Ok(Parser {
            tokenizer,
            current_token,
        })
    }
    pub fn parse(&mut self) -> CalcResult<Node> {
        self.parse_expression(OperatorPrecedence::Default)
    }
}

impl<'a> Parser<'a> {
    fn next_token(&mut self) -> CalcResult<()> {
        self.current_token = self.tokenizer.next().ok_or_else(|| {
            CalcError::UnexpectedChar(self.tokenizer.get_unexpected_char().unwrap())
        })?;
        Ok(())
    }

    fn parse_expression(&mut self, precedence: OperatorPrecedence) -> CalcResult<Node> {
        // TODO: (3)解析表达式}_
        let mut expr = self.parse_number_or_expression()?;
        // 判断当前的Token
        while precedence < self.current_token.get_precedence() {
            expr = self.parse_binary_expression(expr)?;
        }
        Ok(expr)
    }

    fn parse_binary_expression(&mut self, left_expr: Node) -> CalcResult<Node> {
        // TODO: (3)解析二元表达式
        match self.current_token {
            Token::Add => {
                self.next_token()?;
                let right_expr = self.parse_expression(OperatorPrecedence::AddOrSubstract)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Substract => {
                self.next_token()?;
                let right_expr = self.parse_expression(OperatorPrecedence::AddOrSubstract)?;
                Ok(Node::Substract(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Multiply => {
                self.next_token()?;
                let right_expr = self.parse_expression(OperatorPrecedence::MultiplyOrDivide)?;
                Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Divide => {
                self.next_token()?;
                let right_expr = self.parse_expression(OperatorPrecedence::MultiplyOrDivide)?;
                Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Caret => {
                self.next_token()?;
                let right_expr = self.parse_expression(OperatorPrecedence::Power)?;
                Ok(Node::Power(Box::new(left_expr), Box::new(right_expr)))
            }
            _ => unreachable!("无法解析二元表达式"),
        }
    }

    fn parse_number_or_expression(&mut self) -> CalcResult<Node> {
        // TODO: (3)解析数字
        match self.current_token {
            Token::Number(number) => {
                self.next_token()?;
                Ok(Node::Number(number))
            }
            Token::Substract => {
                self.next_token()?;
                let expr = self.parse_expression(OperatorPrecedence::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }
            Token::LeftParen => {
                self.next_token()?;
                let expr = self.parse_expression(OperatorPrecedence::Default)?;
                if self.current_token != Token::RightParen {
                    if self.current_token == Token::EOF {
                        return Err(CalcError::InvalidOperator(String::from("不完整的表达式")));
                    }
                    return Err(CalcError::InvalidOperator(format!(
                        "缺少右括号,但是遇到了 '{}'",
                        self.current_token
                    )));
                }

                self.next_token()?;
                Ok(expr)
            }
            _ => {
                if self.current_token == Token::EOF {
                    return Err(CalcError::InvalidOperator(String::from("不完整的表达式")));
                }
                Err(CalcError::InvalidOperator(format!(
                    "期望数组或表达式，但是遇到了 '{}'",
                    self.current_token
                )))
            }
        }
    }
}

#[cfg(test)]

mod tests {
    use rust_decimal::{Decimal, dec};

    use super::*;
    #[test]
    fn test_add_or_multiply() {
        let mut parser = Parser::new("1+   2 * 3").unwrap();
        assert_eq!(
            parser.parse(),
            Ok(Node::Add(
                Box::new(Node::Number(Decimal::from(1))),
                Box::new(Node::Multiply(
                    Box::new(Node::Number((dec!(2)))),
                    Box::new(Node::Number((dec!(3))))
                ))
            ))
        )
    }
}
