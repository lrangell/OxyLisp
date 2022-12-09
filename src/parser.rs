use crate::types::codeChunk::CodeChunk;
use crate::types::*;
use std::vec::Vec;

pub fn ast(tokens: Vec<Tokens>) -> Option<Form> {
    debug!("AST Tokens:\n{:?}", tokens);
    let (toCall, args) = tokens.split_first()?;
    if let Tokens::Literal(Literal::String(s)) = toCall {
        let code = CodeChunk {
            tokens: args.to_vec(),
        };
        let args: Vec<Form> = code.into_iter().collect();
        // let expression = Form::Expression((s.to_string(), args));
        debug!(
            "ast: Expression:\nSymbol: {}\n Args: {:?} ",
            s.to_string(),
            args
        );
        let expression = Form::Expression((s.to_string(), args));
        return Some(expression);
    }
    debug!("Unable to parse tokens: {:?}", tokens);
    None
}
