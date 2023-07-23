use crate::parser::Rule;

use std::{fmt, write};
#[derive(Debug, Copy, Clone)]
pub enum Operator{
    And,
    Or,
    Xor,
    Plus,
    Minus,
    Mul,
}

#[derive(Debug, Copy, Clone)]
pub enum UnaryOperator{
    Minus,
    Not,
}

#[derive(Debug, Clone)]
pub enum Node{
    Immediate(i32),
    Cont(char, i32),
    UnaryExpr {
        op: UnaryOperator,
        child: Box<Node>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::And => write!(f, "&"),
            Operator::Or => write!(f, "|"),
            Operator::Xor => write!(f, "^"),
            Operator::Mul => write!(f, "*"),
        }
    }
}

impl fmt::Display for UnaryOperator{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self{
            UnaryOperator::Minus => write!(f, "-"),
            UnaryOperator::Not => write!(f, "~"),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Node::Cont(set, _status) => write!(f, "{}", set),
            Node::UnaryExpr { op, child } => write!(f, "({}{})", op, child),
            Node::BinaryExpr { op, lhs, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
            Node::Immediate(number) => write!(f, "{}", number),
        }
    }
}

pub fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule(){
        Rule::Expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::UnaryExpr => {
            let mut pair = pair.into_inner();
            let op = pair.next().unwrap();
            let child = pair.next().unwrap();
            let child = build_ast_from_term(child);
            parse_unary_expr(op, child)            
        }
        Rule::BinaryExpr => {
            let mut pair = pair.into_inner();
            let lhspair = pair.next().unwrap();
            let mut lhs = build_ast_from_term(lhspair);
            let mut op = pair.next().unwrap();
            let rhspair = pair.next().unwrap();
            let mut rhs = build_ast_from_term(rhspair);
            let mut retval = parse_binary_expr(op, lhs, rhs);
            loop {
                let pair_buf = pair.next();
                if pair_buf != None {
                    op = pair_buf.unwrap();
                    lhs = retval;
                    rhs = build_ast_from_term(pair.next().unwrap());
                    retval = parse_binary_expr(op, lhs, rhs);
                } else {
                    return retval;
                }
            }
        }
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

fn build_ast_from_term(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::SET => {
            let istr = pair.as_str();
            let (sign, _istr) = match &istr[..1]{
                "-" => (-1, &istr[1..]),
                "~" => (1, &istr[1..]),
                _ => (1, istr),
            };
            // HACK: for now we use the value of the char as the number for node 
            let status_to_set: u32 = _istr.chars().nth(0).unwrap() as u32;
            let set_name: char = _istr.chars().nth(0).unwrap();
            Node::Cont(set_name, sign*(status_to_set as i32))
        },
        Rule::Int => {
            let istr = pair.as_str();
            let (sign, istr) = match &istr[..1]{
                "-" => (-1, &istr[1..]),
                "~" => (-1, &istr[1..]),
                _ => (1, istr),
            };
            let status_to_set: i32 = istr.parse().unwrap();
            Node::Immediate(sign * status_to_set)
        }
        Rule::Expr => build_ast_from_expr(pair),
        unknown => panic!("Unknown term: {:?}", unknown),
    }
}

fn parse_binary_expr(pair: pest::iterators::Pair<Rule>, lhs: Node, rhs: Node) -> Node{
     Node::BinaryExpr {
        op: match pair.as_str() {
            "&" => Operator::And,
            "|" => Operator::Or,
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "^" => Operator::Xor,
            "*" => Operator::Mul,
            _ => unreachable!(),
        },
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

fn parse_unary_expr(pair: pest::iterators::Pair<Rule>, child: Node) -> Node{
    Node::UnaryExpr { op: 
        match pair.as_str(){
            "-" => UnaryOperator::Minus,
            "~" => UnaryOperator::Not,
            _ => unreachable!(),
    }, child: Box::new(child)}
}
