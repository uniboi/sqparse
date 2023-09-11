use crate::lexer::parse_str::ParseStr;
use crate::token::TerminalToken;

pub fn try_symbol(val: ParseStr) -> Option<(TerminalToken, ParseStr)> {
    TerminalToken::SYMBOLS
        .iter()
        .find(|(_, token_val)| val.as_str().starts_with(token_val))
        .map(|(token, token_val)| (*token, val.from(token_val.len())))
}
