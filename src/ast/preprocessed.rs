use crate::token::Token;

use super::Expression;

#[derive(Debug, Clone)]
pub struct PreprocesserIfExpression<'s, T> {
    pub if_: &'s Token<'s>,
    pub else_: Option<&'s Token<'s>>,
    pub elseif: Option<&'s Token<'s>>,
    pub endif: &'s Token<'s>,
    pub if_condition: Expression<'s>,
    pub content: T,
    pub elseif_content: Option<Box<PreprocesserIfExpression<'s, T>>>,
    pub else_content: Option<T>,
}

// #[derive(Debug, Clone)]
// pub enum Preprocessable<'s, T> {
//     PREPROCESSED(Box<PreprocesserIfExpression<'s, Preprocessable<'s, T>>>),
//     UNCONDITIONAL(T),
// }

#[derive(Debug, Clone)]
pub enum Preprocessable<'s, T> {
    PREPROCESSED(Box<PreprocesserIfExpression<'s, Vec<Preprocessable<'s, T>>>>),
    UNCONDITIONAL(T),
}

#[derive(Debug, Clone)]
pub struct PreprocesserIfExpressionList<'s, T> {
    pub if_: &'s Token<'s>,
    pub else_: Option<&'s Token<'s>>,
    pub elseif: Option<&'s Token<'s>>,
    pub endif: &'s Token<'s>,
    pub if_condition: Expression<'s>,
    pub content: Vec<T>,
    pub elseif_content: Option<Box<PreprocesserIfExpression<'s, T>>>,
    pub else_content: Option<T>,
}