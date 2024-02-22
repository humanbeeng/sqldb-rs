#[allow(dead_code)]
use crate::lexer::Token;

#[derive(Debug)]
pub struct Ast {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Insert {
        table: Token,
        values: Vec<Expression>,
    },
    Select {
        from: Token,
        items: Vec<Expression>,
    },
    Create {
        name: Token,
        cols: Vec<ColDefinition>,
    },
}

#[derive(Debug)]
pub struct ColDefinition {
    name: Token,
    data_type: Token,
}

#[derive(Debug)]
pub struct Expression {
    literal: Token,
    kind: ExpressionKind,
}

#[derive(Debug)]
pub enum ExpressionKind {
    Literal,
}
