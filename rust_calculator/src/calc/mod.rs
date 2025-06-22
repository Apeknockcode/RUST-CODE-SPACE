use rust_decimal::Decimal;

use crate::calc::error::CalcResult;

mod ast;
mod error;
mod parser;
mod token;
mod tokenizer;

pub fn calculate(expression: &str) -> CalcResult<Decimal> {
    let mut parser = parser::Parser::new(expression)?;
    let ast = parser.parse()?;
    Ok(ast.evaluate())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_or_multiply() {
        use rust_decimal::dec;
        assert_eq!(calculate("1+2").unwrap(), dec!(3));
        assert_eq!(calculate("1+2*3").unwrap(), dec!(7));
        assert_eq!(calculate("1+2*3+4").unwrap(), dec!(11));
        assert_eq!(calculate("4/2+2*3+4").unwrap(), dec!(12));
        assert_eq!(calculate("-1 * 2").unwrap(), dec!(-2));
        assert_eq!(calculate("-1 + 2").unwrap(), dec!(1));
         assert_eq!(calculate("-1 - 2").unwrap(), dec!(-3));
        assert_eq!(calculate("1^5").unwrap(), dec!(1));
        assert_eq!(calculate("1^5+2").unwrap(), dec!(3));
         assert_eq!(calculate("(3-(2+4)*2/2)^2").unwrap(), dec!(9));
    }
}
