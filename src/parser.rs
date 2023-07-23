use pest::Parser;

use crate::Node;

#[derive(pest_derive::Parser)]
#[grammar="grammar.pest"]
pub struct LogicParser;

pub fn parse(source: &str) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>>{
    let mut ast = vec![];
    let pairs = LogicParser::parse(Rule::Expr, source)?;
    for pair in pairs{
        if let Rule::Expr = pair.as_rule(){
            ast.push(crate::ast::build_ast_from_expr(pair))
        }
    }
    Ok(ast)
}
