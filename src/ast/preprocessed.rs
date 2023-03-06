use crate::token::Token;

use super::Expression;

#[derive(Debug, Clone)]
pub struct PreprocessorIfExpression<'s, T> {
    pub if_: &'s Token<'s>,
    pub else_: Option<PreprocessorElseExpression<'s, T>>,
    pub elseif: Option<Box<PreprocessorElseIfExpression<'s, T>>>,
    pub endif: &'s Token<'s>,
    pub condition: Box<Expression<'s>>,
    pub content: T,
}

#[derive(Debug, Clone)]
pub struct PreprocessorElseExpression<'s, T> {
    pub else_: &'s Token<'s>,
    pub content: T,
}

#[derive(Debug, Clone)]
pub struct PreprocessorElseIfExpression<'s, T> {
    pub elseif_: &'s Token<'s>,
    pub condition: Box<Expression<'s>>,
    pub content: T,
    pub elseif: Option<Box<PreprocessorElseIfExpression<'s, T>>>,
}

#[derive(Debug, Clone)]
pub enum Preprocessable<'s, T> {
    PREPROCESSED(Box<PreprocessorIfExpression<'s, Vec<Preprocessable<'s, T>>>>),
    UNCONDITIONAL(T),
}
