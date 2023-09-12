use crate::{
    ast::{
        RenderDefinition, RenderParameter, RenderParameters, RenderParent, RenderParentStatement,
        RuiParam, RuiRenderDefinitionsStatement, Self_, SeparatedListTrailing0, Topology,
    },
    parser::{parse_result_ext::ParseResultExt, variable::var_initializer},
    token::TerminalToken,
    ContextType, ParseErrorType,
};

use super::{
    identifier::identifier, token_list::TokenList, token_list_ext::TokenListExt, type_::type_,
    ParseResult,
};

pub fn rui_definition_params(tokens: TokenList) -> ParseResult<SeparatedListTrailing0<RuiParam>> {
    let (tokens, params_list) = tokens
        .separated_list_trailing0(rui_definition_param, |tokens| {
            tokens.terminal(TerminalToken::Comma)
        })?;

    Ok((tokens, params_list))
}

pub fn rui_definition_param(tokens: TokenList) -> ParseResult<RuiParam> {
    type_(tokens)
        .and_then(|(tokens, type_)| identifier(tokens).map_val(|name| (type_, name)))
        .determines(|tokens, (type_, name)| {
            let (tokens, initializer) = var_initializer(tokens)?;
            Ok((
                tokens,
                RuiParam {
                    type_,
                    name,
                    initializer,
                },
            ))
        })
}

pub fn rui_render_definitions(tokens: TokenList) -> ParseResult<RuiRenderDefinitionsStatement> {
    tokens
        .terminal(TerminalToken::OpenSquare)
        .determines_and_opens(
            ContextType::RuiRenderDefinitionList,
            |tokens| tokens.terminal(TerminalToken::CloseSquare),
            |tokens, open, close| {
                let (tokens, defs) = tokens.many_until_ended(rui_render_definition)?;
                Ok((tokens, RuiRenderDefinitionsStatement { open, defs, close }))
            },
        )
}

pub fn rui_render_definition(tokens: TokenList) -> ParseResult<RenderDefinition> {
    type_(tokens).determines(|tokens, type_| {
        let (tokens, name) = identifier(tokens)?;
        let (tokens, parent) = rui_render_parent(tokens)?;
        let (tokens, params) = rui_render_parameters(tokens)?;
        Ok((
            tokens,
            RenderDefinition {
                type_,
                name,
                parent,
                params,
            },
        ))
    })
}

pub fn rui_render_parameters(tokens: TokenList) -> ParseResult<RenderParameters> {
    tokens
        .terminal(TerminalToken::OpenBrace)
        .determines_and_opens(
            ContextType::RuiRenderParameterList,
            |tokens| tokens.terminal(TerminalToken::CloseBrace),
            |tokens, open, close| {
                let (tokens, params) = tokens
                    .separated_list_trailing0(rui_render_parameter, |tokens| {
                        tokens.terminal(TerminalToken::Comma)
                    })?;
                Ok((
                    tokens,
                    RenderParameters {
                        open,
                        params,
                        close,
                    },
                ))
            },
        )
}

pub fn rui_render_parameter(tokens: TokenList) -> ParseResult<RenderParameter> {
    identifier(tokens)
        .and_then(|(tokens, name)| {
            var_initializer(tokens).map_val(|initializer| (name, initializer))
        })
        .determines(|tokens, (name, initializer)| {
            Ok((tokens, RenderParameter { name, initializer }))
        })
}

pub fn rui_render_parent(tokens: TokenList) -> ParseResult<RenderParentStatement> {
    tokens
        .terminal(TerminalToken::Less)
        .determines(|tokens, open| {
            let (tokens, parent) = rui_render_parent_type(tokens)?;
            let (tokens, close) = tokens.terminal(TerminalToken::Greater)?;
            Ok((
                tokens,
                RenderParentStatement {
                    open,
                    parent,
                    close,
                },
            ))
        })
}

pub fn rui_render_parent_type(tokens: TokenList) -> ParseResult<RenderParent> {
    self_(tokens)
        .map_val(RenderParent::Self_)
        .or_try(|| topology(tokens).map_val(RenderParent::Topology))
        .or_try(|| identifier(tokens).map_val(RenderParent::Identifier))
        .or_error(|| tokens.error(ParseErrorType::ExpectedRuiParent))
}

pub fn self_(tokens: TokenList) -> ParseResult<Self_> {
    tokens
        .terminal(TerminalToken::Self_)
        .map_val(|self_| Self_ { self_ })
}

pub fn topology(tokens: TokenList) -> ParseResult<Topology> {
    tokens
        .terminal(TerminalToken::Topology)
        .map_val(|topology| Topology { topology })
}
