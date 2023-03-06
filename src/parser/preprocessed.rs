use crate::{
    ast::{
        Precedence, PreprocessorElseExpression, PreprocessorElseIfExpression,
        PreprocessorIfExpression, Statement,
    },
    token::TerminalToken,
    ContextType,
};

use super::{
    expression::expression, parse_result_ext::ParseResultExt, statement, token_list::TokenList,
    token_list_ext::TokenListExt, ParseResult,
};

pub fn many_preprocessed_if_contents<T>(
    tokens: TokenList,
    stm: fn(TokenList) -> ParseResult<T>,
) -> ParseResult<Vec<T>> {
    tokens.many_until(
        |tokens| {
            tokens.is_ended()
                || tokens.terminal(TerminalToken::PreprocessorElseIf).is_ok()
                || tokens.terminal(TerminalToken::PreprocessorElse).is_ok()
        },
        stm,
    )
}

pub fn preprocessed_if<'s, T, FnParser: Fn(TokenList<'s>) -> ParseResult<'s, T>>(
    tokens: TokenList<'s>,
    stm: FnParser,
) -> ParseResult<Box<PreprocessorIfExpression<T>>> {
    tokens
        .terminal(TerminalToken::PreprocessorIf)
        .determines_and_opens(
            ContextType::PreProcessorIf,
            |tokens| tokens.terminal(TerminalToken::PreprocessorEndIf),
            |tokens, open, close| {
                let (tokens, condition) = expression(tokens, Precedence::None)?;
                // let (tokens, statements) = tokens.many_until(
                // |tokens| {
                //     tokens.is_ended()
                //         || tokens.terminal(TerminalToken::PreprocessorElseIf).is_ok()
                //         || tokens.terminal(TerminalToken::PreprocessorElse).is_ok()
                // },
                // stm,
                // )?;
                let (tokens, statements) = stm(tokens)?;
                // let (tokens, statements) = (|tokens: TokenList| {
                //     tokens.many_until(
                //         |tokens: TokenList| {
                //             tokens.is_ended()
                //                 || tokens.terminal(TerminalToken::PreprocessorElseIf).is_ok()
                //                 || tokens.terminal(TerminalToken::PreprocessorElse).is_ok()
                //         },
                //         statement,
                //     )
                // })(tokens)?;

                let (tokens, elseif) = preprocessed_elseif(tokens, &stm).maybe(tokens)?;
                let (tokens, else_) = preprocessed_else(tokens, &stm).maybe(tokens)?;

                Ok((
                    tokens,
                    Box::new(PreprocessorIfExpression {
                        if_: open,
                        else_,
                        elseif,
                        endif: close,
                        condition,
                        content: statements,
                    }),
                ))
            },
        )
}

pub fn preprocessed_elseif<'s, T, FnParser: Fn(TokenList<'s>) -> ParseResult<'s, T>>(
    tokens: TokenList<'s>,
    stm: FnParser,
) -> ParseResult<Box<PreprocessorElseIfExpression<T>>> {
    tokens
        .terminal(TerminalToken::PreprocessorElseIf)
        .determines(|tokens, elseif_| {
            let (tokens, condition) = expression(tokens, Precedence::None)?;
            // let (tokens, statements) = tokens.many_until(
            //     |tokens| {
            //         tokens.is_ended()
            //             || tokens.terminal(TerminalToken::PreprocessorElseIf).is_ok()
            //             || tokens.terminal(TerminalToken::PreprocessorElse).is_ok()
            //     },
            //     stm,
            // )?;
            let (tokens, statements) = stm(tokens)?;
            let (tokens, elseif) = preprocessed_elseif(tokens, stm).maybe(tokens)?;
            Ok((
                tokens,
                Box::new(PreprocessorElseIfExpression {
                    condition,
                    content: statements,
                    elseif,
                    elseif_,
                }),
            ))
        })
}

pub fn preprocessed_else<'s, T, FnParser: Fn(TokenList<'s>) -> ParseResult<'s, T>>(
    tokens: TokenList<'s>,
    stm: FnParser,
) -> ParseResult<PreprocessorElseExpression<T>> {
    tokens
        .terminal(TerminalToken::PreprocessorElse)
        .determines(|tokens, else_| {
            // let (tokens, statements) = tokens.many_until_ended(stm)?;
            let (tokens, statements) = stm(tokens)?;
            Ok((
                tokens,
                PreprocessorElseExpression {
                    else_,
                    content: statements,
                },
            ))
        })
}
