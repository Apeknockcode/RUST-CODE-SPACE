use rust_decimal::{Decimal, MathematicalOps};


#[derive(Debug, PartialEq, Clone )]
pub enum Node{
    Add(Box<Node>, Box<Node>),
    Substract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Power(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Number(Decimal),
}

impl Node{
    // TODO: 实现一个评估的方法
    pub fn evaluate(&self) -> Decimal{
        use Node::*;
        match self {
            Add(left, right) => left.evaluate() + right.evaluate(),
            Substract(left, right) => left.evaluate() - right.evaluate(),
            Multiply(left, right) => left.evaluate() * right.evaluate(),
            Divide(left, right) => left.evaluate() / right.evaluate(),
            Power(left, right) => left.evaluate().powd(right.evaluate()),
            Negative(node) => -node.evaluate(),
            Number(n) => *n,
        }
    }
}

#[cfg(test)]
mod test{
    use rust_decimal::dec;

    use super::*;
    #[test]
    fn test_node_evaluate(){
        assert_eq!(Node::Number(Decimal::from(100)).evaluate(), Decimal::from(100));
        assert_eq!(Node::Add(Box::new(Node::Number(Decimal::from(100))), Box::new(Node::Number(Decimal::from(200)))).evaluate(), Decimal::from(300));
        assert_eq!(Node::Substract(Box::new(Node::Number(Decimal::from(100))), Box::new(Node::Number(Decimal::from(200)))).evaluate(), Decimal::from(-100));
        assert_eq!(Node::Multiply(Box::new(Node::Number(Decimal::from(100))), Box::new(Node::Number(Decimal::from(200)))).evaluate(), Decimal::from(20000));
        assert_eq!(Node::Divide(Box::new(Node::Number(Decimal::from(dec!(1)))), Box::new(Node::Number(Decimal::from(dec!(2))))).evaluate(), Decimal::from(dec!(0.5)));
        assert_eq!(Node::Power(Box::new(Node::Number(Decimal::from(1))), Box::new(Node::Number(Decimal::from(2)))).evaluate(), Decimal::from(1));
        assert_eq!(Node::Negative(Box::new(Node::Number(Decimal::from(100)))).evaluate(), Decimal::from(-100));
    }
}