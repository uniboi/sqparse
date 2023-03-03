use crate::ast::{
    Precedence, Preprocessable, PreprocesserIfExpression, StructDefinition, StructProperty,
};
use crate::parser::identifier::identifier;
use crate::parser::parse_result_ext::ParseResultExt;
use crate::parser::token_list::TokenList;
use crate::parser::token_list_ext::TokenListExt;
use crate::parser::type_::type_;
use crate::parser::variable::var_initializer;
use crate::parser::ParseResult;
use crate::token::TerminalToken;
use crate::ContextType;

use super::expression::expression;

pub fn struct_definition(tokens: TokenList) -> ParseResult<StructDefinition> {
    tokens.terminal(TerminalToken::OpenBrace).opens(
        ContextType::Span,
        |tokens| tokens.terminal(TerminalToken::CloseBrace),
        |tokens, open, close| {
            tokens
                .many_until_ended(possibly_preprocessed_struct_property)
                .map_val(|properties| StructDefinition {
                    open,
                    properties,
                    close,
                })
        },
    )
}

pub fn possibly_preprocessed_struct_property(
    tokens: TokenList,
) -> ParseResult<Preprocessable<StructProperty>> {
    preprocessed_struct_properties(tokens).or_try(|| struct_property(tokens))
}

pub fn struct_property(tokens: TokenList) -> ParseResult<Preprocessable<StructProperty>> {
    type_(tokens).determines(|tokens, type_| {
        let (tokens, name) = identifier(tokens)?;
        let (tokens, initializer) = var_initializer(tokens).maybe(tokens)?;
        let (tokens, comma) = tokens.terminal(TerminalToken::Comma).maybe(tokens)?;
        Ok((
            tokens,
            Preprocessable::UNCONDITIONAL(StructProperty {
                type_,
                name,
                initializer,
                comma,
            }),
        ))
    })
}

pub fn preprocessed_struct_properties(
    tokens: TokenList,
) -> ParseResult<Preprocessable<StructProperty>> {
    tokens
        .terminal(TerminalToken::PreprocessorIf)
        .determines_and_opens(
            ContextType::PreProcessorIf,
            |tokens| tokens.terminal(TerminalToken::PreprocessorEndIf),
            |tokens, if_, endif| {
                let (tokens, condition) = expression(tokens, Precedence::None)?;
                let (tokens, properties) =
                    tokens.many_until_ended(possibly_preprocessed_struct_property)?;
                Ok((
                    tokens,
                    Preprocessable::PREPROCESSED(Box::new(PreprocesserIfExpression {
                        if_,
                        else_: None,
                        elseif: None,
                        endif,
                        if_condition: *condition,
                        content: properties,
                        elseif_content: None,
                        else_content: None,
                    })),
                ))
            },
        )
}
