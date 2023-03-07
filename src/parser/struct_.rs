use crate::ast::{Preprocessable, StructDefinition, StructProperty};
use crate::parser::identifier::identifier;
use crate::parser::parse_result_ext::ParseResultExt;
use crate::parser::token_list::TokenList;
use crate::parser::token_list_ext::TokenListExt;
use crate::parser::type_::type_;
use crate::parser::variable::var_initializer;
use crate::parser::ParseResult;
use crate::token::TerminalToken;
use crate::ContextType;

use super::preprocessed::{preprocessed_if, preprocessed_if_contents_terminal};

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
    let (tokens, preprocessed) = preprocessed_if(tokens, |tokens| {
        tokens.many_until(
            |tokens| preprocessed_if_contents_terminal(tokens),
            possibly_preprocessed_struct_property,
        )
    })?;
    Ok((tokens, Preprocessable::PREPROCESSED(preprocessed)))
}
