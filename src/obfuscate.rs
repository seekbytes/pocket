use crate::{Node, Operator, ast::UnaryOperator};
pub struct Obfuscate;

impl Obfuscate{
    pub fn new() -> Self{
        Self
    }

    pub fn obfuscate_node(&self, node: &Node) -> Node{
        match node {
            Node::Cont(_set, _status) => Node::Cont(*_set, *_status),
            Node::Immediate(number) => Node::Immediate(*number),
            Node::UnaryExpr { op, child } => {
                let _child = self.obfuscate_node(child);
                match op {
                    UnaryOperator::Not => Node::UnaryExpr { op: UnaryOperator::Not, child: Box::new(_child) },
                    UnaryOperator::Minus => Node::UnaryExpr { op: UnaryOperator::Minus, child: Box::new(_child) },
                }
            },
            Node::BinaryExpr { op, lhs, rhs } => {
                let left_value = self.obfuscate_node(lhs);
                let right_value = self.obfuscate_node(rhs);
                match op{
                    Operator::And => {
                        Node::BinaryExpr { op: Operator::Minus, 
                            lhs: Box::new(Node::BinaryExpr { op: Operator::Plus, lhs: Box::new(left_value.clone()), rhs: Box::new(right_value.clone())}), 
                            rhs: Box::new(Node::BinaryExpr { op: Operator::Or, lhs: Box::new(left_value.clone()), rhs: Box::new(right_value.clone()) }) }
                    },
                    Operator::Plus => {
                        Node::BinaryExpr { op: Operator::Plus, 
                            lhs: Box::new(Node::BinaryExpr { op: Operator::And, lhs: Box::new(left_value.clone()), rhs: Box::new(right_value.clone()) }), 
                            rhs: Box::new(Node::BinaryExpr { op: Operator::Or, lhs: Box::new(left_value.clone()), rhs: Box::new(right_value.clone()) }) }
                    },
                    Operator::Xor => {
                        Node::BinaryExpr { op: Operator::Minus, 
                            lhs: Box::new(Node::BinaryExpr { op: Operator::Or, lhs: Box::new(left_value.clone()), rhs: Box::new(right_value.clone()) }), 
                            rhs: Box::new(Node::BinaryExpr { op: Operator::And, lhs: Box::new(left_value.clone()), rhs: Box::new(right_value.clone()) }) }
                    },
                    Operator::Minus => {
                        Node::BinaryExpr { op: Operator::Plus,
                         lhs: Box::new(Node::BinaryExpr { 
                            op: Operator::Xor, 
                            lhs: Box::new(left_value.clone()), 
                            rhs: Box::new(Node::UnaryExpr { op: UnaryOperator::Minus, child: Box::new(right_value.clone()) })
                        }),
                        rhs: Box::new(Node::BinaryExpr { 
                            op: Operator::Mul, 
                            lhs: Box::new(Node::Immediate(2)), rhs: Box::new(Node::BinaryExpr { 
                                op: Operator::And, lhs: Box::new(left_value.clone()), rhs: Box::new(Node::UnaryExpr 
                                    { op: UnaryOperator::Minus, child: Box::new(right_value.clone()) }) }
                                ) 
                        }) }
                    },
                    Operator::Or => {
                        Node::BinaryExpr { op: Operator::Plus,
                        lhs: Box::new(left_value.clone()),
                        rhs: Box::new(Node::BinaryExpr { 
                            op: Operator::Plus, 
                            lhs: Box::new(right_value.clone()),
                            rhs: Box::new(Node::BinaryExpr { 
                                op: Operator::Plus, 
                                lhs: Box::new(Node::Immediate(1)), 
                                rhs: Box::new(Node::BinaryExpr { 
                                    op: Operator::Or, 
                                    lhs: Box::new(Node::UnaryExpr { op: UnaryOperator::Not, child: Box::new(left_value.clone()) }), 
                                    rhs: Box::new(Node::UnaryExpr { op: UnaryOperator::Not, child: Box::new(right_value.clone()) }) 
                            }) })
                        }) }
                    },
                    Operator::Mul => {
                        Node::BinaryExpr { op: Operator::Mul, lhs: Box::new(left_value.clone()), rhs: Box::new(right_value.clone())}
                    }
                }

            },
        }
    }
}