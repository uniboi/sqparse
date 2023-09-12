use super::{Identifier, SeparatedListTrailing0, Type, VarInitializer};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct RuiParam<'s> {
    pub type_: Type<'s>,
    pub name: Identifier<'s>,
    pub initializer: VarInitializer<'s>,
}

#[derive(Debug, Clone)]
pub enum RenderParent<'s> {
    Self_(Self_<'s>),
    Topology(Topology<'s>),
    Identifier(Identifier<'s>),
}

#[derive(Debug, Clone)]
pub struct RenderDefinition<'s> {
    pub type_: Type<'s>,
    pub name: Identifier<'s>,
    pub parent: RenderParentStatement<'s>,
    pub params: RenderParameters<'s>,
}

#[derive(Debug, Clone)]
pub struct RenderParentStatement<'s> {
    pub open: &'s Token<'s>,
    pub parent: RenderParent<'s>,
    pub close: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct Self_<'s> {
    pub self_: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct Topology<'s> {
    pub topology: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct RenderParameters<'s> {
    pub open: &'s Token<'s>,
    pub params: SeparatedListTrailing0<'s, RenderParameter<'s>>,
    pub close: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct RenderParameter<'s> {
    pub name: Identifier<'s>,
    pub initializer: VarInitializer<'s>,
}
