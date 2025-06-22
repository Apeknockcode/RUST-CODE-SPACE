use std::iter::Peekable;
use std::str::Chars;
use crate::calc::token::Token;

// TODO: (2)分词器
pub struct Tokenizer<'a> {
    expression: Peekable<Chars<'a>>, //引用
    reached_end: bool, // 定于状态类型的变量
    unexpected_char: Option<char>// 声明一个变量表示未知的字符
}


impl<'a> Tokenizer<'a> {
    pub fn new(expression: &'a str) -> Self {
        Tokenizer {
            expression: expression.chars().peekable(),
            reached_end: false,
            unexpected_char: None,
        }
    }

    // 返回导致错误的字符
    pub fn get_unexpected_char(&self) -> Option<char> {
        self.unexpected_char
    }

    //
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
     fn next(&mut self) -> Option<Self::Item> {
        if self.reached_end {
            return None;
        }

        let next_chr = self.expression.next();
        match next_chr {
            Some(chr) if chr.is_numeric() => {
                let mut number = String::from(chr);
                // while let Some(next_chr) = self.expression.peek() {
                //     if next_chr.is_numeric() {
                //         number.push(self.expression.next().unwrap());
                //     }else{
                //         break;
                //     }
                // }
                while let Some(next_chr) = self.expression.next_if(|c| c.is_numeric()){
                    number.push(next_chr);
                }
                Some(Token::Number(number.parse().unwrap()))
            }

            // 匹配是不是空白字符
            Some(chr) if chr.is_whitespace() => {
                while let Some(_) = self.expression.next_if(|c| c.is_whitespace()) {}
                self.next()
            }
            // 匹配是不是运算符
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Substract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            None => {
                self.reached_end = true;
                Some(Token::EOF)
            }
            Some(chr) => {
                self.unexpected_char = Some(chr);
                None
            }
        }
     }
}


// TODO: (3)测试分词器
#[cfg(test)]
mod tests {
    use rust_decimal::dec;

    use super::*;

    #[test]
    fn test_tokenizer() {
        let expression = "1 +    2 * 3";
        let mut tokenizer = Tokenizer::new(expression);
        assert_eq!(
            tokenizer.collect::<Vec<Token>>(),
            vec![
                Token::Number(dec!(1)),
                Token::Add,
                Token::Number(dec!(2)),
                Token::Multiply,
                Token::Number(dec!(3)),
                Token::EOF
            ]
        );
    }
}