use crate::ast::{ArrayValue, Precedence, Preprocessable};
use crate::parser::expression::expression;
use crate::parser::parse_result_ext::ParseResultExt;
use crate::parser::token_list_ext::TokenListExt;
use crate::parser::{ParseResult, TokenList};
use crate::token::TerminalToken;

use super::preprocessed::{preprocessed_if, preprocessed_if_contents_terminal};

pub fn possibly_preprocessed_array_value(
    tokens: TokenList,
) -> ParseResult<Preprocessable<ArrayValue>> {
    preprocessed_array_value(tokens).or_try(|| array_value(tokens))
}

pub fn array_value(tokens: TokenList) -> ParseResult<Preprocessable<ArrayValue>> {
    let (tokens, value) = expression(tokens, Precedence::Comma)?;
    let (tokens, separator) = tokens.terminal(TerminalToken::Comma).maybe(tokens)?;
    Ok((
        tokens,
        Preprocessable::UNCONDITIONAL(ArrayValue { value, separator }),
    ))
}

pub fn preprocessed_array_value(tokens: TokenList) -> ParseResult<Preprocessable<ArrayValue>> {
    let (tokens, preprocessed) = preprocessed_if(tokens, |tokens| {
        tokens.many_until(
            |tokens| preprocessed_if_contents_terminal(tokens),
            possibly_preprocessed_array_value,
        )
    })?;
    Ok((tokens, Preprocessable::PREPROCESSED(preprocessed)))
}
