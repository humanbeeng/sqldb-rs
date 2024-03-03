#[allow(dead_code)]
use crate::lexer::Token;

#[derive(Debug)]
pub struct Ast {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct Statement {
    pub create: Option<Create>,
    pub insert: Option<Insert>,
    pub select: Option<Select>,
}

#[derive(Debug)]
pub struct Insert {
    pub table: Token,
    pub values: Vec<Expression>,
}

#[derive(Debug)]
pub struct Select {
    pub from: Token,
    pub items: Vec<Expression>,
}

#[derive(Debug)]
pub struct Create {
    pub name: Token,
    pub cols: Vec<ColDefinition>,
}

#[derive(Debug)]
pub struct ColDefinition {
    pub name: Token,
    pub data_type: Token,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Expression {
    pub literal: Token,
    pub kind: ExpressionKind,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExpressionKind {
    Literal,
}
