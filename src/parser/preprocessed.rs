use crate::{
    ast::{
        Precedence, PreprocessorElseExpression, PreprocessorElseIfExpression,
        PreprocessorIfExpression,
    },
    token::TerminalToken,
    ContextType,
};

use super::{
    expression::expression, parse_result_ext::ParseResultExt, token_list::TokenList,
    token_list_ext::TokenListExt, ParseResult,
};

pub fn preprocessed_if_contents_terminal(tokens: TokenList) -> bool {
    tokens.is_ended()
        || tokens.terminal(TerminalToken::PreprocessorElseIf).is_ok()
        || tokens.terminal(TerminalToken::PreprocessorElse).is_ok()
}

pub fn preprocessed_if<'s, T, FnParser: Fn(TokenList<'s>) -> ParseResult<'s, T>>(
    tokens: TokenList<'s>,
    parser: FnParser,
) -> ParseResult<Box<PreprocessorIfExpression<T>>> {
    tokens
        .terminal(TerminalToken::PreprocessorIf)
        .determines_and_opens(
            ContextType::PreProcessorIf,
            |tokens| tokens.terminal(TerminalToken::PreprocessorEndIf),
            |tokens, if_, endif| {
                let (tokens, condition) = expression(tokens, Precedence::None)?;
                let (tokens, content) = parser(tokens)?;
                let (tokens, elseif) = preprocessed_elseif(tokens, &parser).maybe(tokens)?;
                let (tokens, else_) = preprocessed_else(tokens, &parser).maybe(tokens)?;

                Ok((
                    tokens,
                    Box::new(PreprocessorIfExpression {
                        if_,
                        else_,
                        elseif,
                        endif,
                        condition,
                        content,
                    }),
                ))
            },
        )
}

pub fn preprocessed_elseif<'s, T, FnParser: Fn(TokenList<'s>) -> ParseResult<'s, T>>(
    tokens: TokenList<'s>,
    parser: FnParser,
) -> ParseResult<Box<PreprocessorElseIfExpression<T>>> {
    tokens
        .terminal(TerminalToken::PreprocessorElseIf)
        .determines(|tokens, elseif_| {
            let (tokens, condition) = expression(tokens, Precedence::None)?;
            let (tokens, content) = parser(tokens)?;
            let (tokens, elseif) = preprocessed_elseif(tokens, parser).maybe(tokens)?;
            Ok((
                tokens,
                Box::new(PreprocessorElseIfExpression {
                    condition,
                    content,
                    elseif,
                    elseif_,
                }),
            ))
        })
}

pub fn preprocessed_else<'s, T, FnParser: Fn(TokenList<'s>) -> ParseResult<'s, T>>(
    tokens: TokenList<'s>,
    parser: FnParser,
) -> ParseResult<PreprocessorElseExpression<T>> {
    tokens
        .terminal(TerminalToken::PreprocessorElse)
        .determines(|tokens, else_| {
            let (tokens, content) = parser(tokens)?;
            Ok((
                tokens,
                PreprocessorElseExpression {
                    else_,
                    content,
                },
            ))
        })
}
