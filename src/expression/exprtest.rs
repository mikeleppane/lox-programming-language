use color_eyre::Result;
use crate::interpreter::error::LoxError;
use crate::tokens::token::{Object, Token};

pub enum ExprTEST {
	Binary(BinaryExprTEST),
	Grouping(GroupingExprTEST),
	Literal(LiteralExprTEST),
	Unary(UnaryExprTEST),
}

pub struct BinaryExprTEST {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

pub struct GroupingExprTEST {
    expression: Box<Expr>,
}

pub struct LiteralExprTEST {
    value: Object,
}

pub struct UnaryExprTEST {
    operator: Token,
    right: Box<Expr>,
}

