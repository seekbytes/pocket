pub struct Intrepreter;
use crate::ast::{Node, Operator, UnaryOperator};

pub trait Compile {
    type Output;

    fn from_ast(ast: Vec<Node>) -> Self::Output;

    fn from_source(source: &str) -> Self::Output {
        let ast: Vec<Node> = crate::parser::parse(source).unwrap();
        Self::from_ast(ast)
    }
}

struct AST {
    v: Vec<Node>,
}

// Helper version to print the AST
fn print_ast(ast: &Vec<Node>){
    for node in ast {
        println!("{}", node);
    }
}


impl Compile for Intrepreter{
    type Output = Result<i32, char>;

    fn from_ast(ast: Vec<Node>) -> Self::Output{

        let mut ret = 0;
        let evaluator = Eval::new();
        let obfuscator = crate::obfuscate::Obfuscate::new();

        // Force the allocation on heap
        let mut ob_ast = Box::new(AST{v: Vec::new()});
        for node in ast{
                // Evaluation of the node
                ret += evaluator.eval(&node);
                // Obfuscate!
                ob_ast.v.push(obfuscator.obfuscate_node(&node));
                println!("Obfuscated version 1: ");
                print_ast(&ob_ast.v);
                
        }

        // TODO: Missing one equality! The first level of obfuscation is not checked.

        for i in 1..5{
            let mut tmp_ast = Box::new(AST{v: Vec::new()});
            let mut ret_ob = 0;
            for node in &ob_ast.v{
                // Valutiamo il valore
                ret_ob += evaluator.eval(&node);
                // Offuschiamo
                tmp_ast.v.push(obfuscator.obfuscate_node(&node));
                println!("Obfuscated version {}: ", i+1);
                print_ast(&tmp_ast.v);
            }
            ob_ast = tmp_ast;
            println!("Result of valued expression: {}", ret_ob);
            assert_eq!(ret, ret_ob);

        }

        println!("End of computation.");
        Ok(ret)
    }
}


struct Eval;
impl Eval{
    pub fn new() -> Self{
        Self
    }

    fn eval(&self, node: &Node) -> i32{
        match node {
            Node::Cont(_set, value) => *value,
            Node::Immediate(value) => *value,
            Node::UnaryExpr { op, child } => {
                let child = self.eval(child);
                return match op{
                    UnaryOperator::Not => !child,
                    UnaryOperator::Minus => -child,
                }
            },
            Node::BinaryExpr { op, lhs, rhs } => {
                let left_value = self.eval(lhs);
                let right_value = self.eval(rhs);
                match op{
                    Operator::And => left_value & right_value,
                    Operator::Or => left_value | right_value,
                    Operator::Xor => left_value ^ right_value,
                    Operator::Plus => left_value + right_value,
                    Operator::Minus=> left_value - right_value,
                    Operator::Mul => left_value * right_value,
                }
            },
        }
    }
}