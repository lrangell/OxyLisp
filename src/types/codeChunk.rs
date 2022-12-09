use crate::lexer::*;
use crate::parser::*;
use crate::types::*;
//
// pub struct CodeChunk {
//     pub tokens: Vec<Tokens>,
// }
//
// impl CodeChunk {
//     pub fn new(code: &str) -> Self {
//         Self {
//             tokens: tokenize(code),
//         }
//     }
// }
//
// impl fmt::Display for CodeChunk {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:?}", self.tokens)
//     }
// }
//
//
// fn tokens_to_form(tokens: &[Tokens]) -> Result<Form, &str> {
//     match tokens.first().ok_or("444")? {
//         Tokens::Bounds(TokenBounds::LeftParen) => {
//             let (tocall, rest) = tokens.split_first().ok_or("aaa")?;
//             if let Tokens::Literal(Literal::String(s)) = tocall {
//                 let (_, args) = rest.split_last().ok_or("333")?;
//                 return Ok(Form::CallExpression((
//                     s.to_string(),
//                     parsetoken2(&args.clone().to_vec()),
//                 )));
//             }
//             Err("444")
//         }
//         Tokens::Bounds(TokenBounds::RightParen) => Err("444"),
//         Tokens::Literal(p) => Ok(p.clone().into()),
//     }
// }
//
// fn parsetoken2(tokens: &Vec<Tokens>) -> Vec<Form> {
//     let mut inside = false;
//     tokens.group_by(|prev, curr| match prev {Tokens::Literal(_) => })
// }
//
// fn parse_expression(tokens: &Vec<Tokens>) -> Result<Form, &str> {
//     let (toCall, args_tokens) = tokens.split_first().ok_or("333")?;
//     if let Tokens::Literal(Literal::String(s)) = toCall {
//         let args = tokens_to_form(args_tokens.to_vec())?;
//         let expression = Form::CallExpression((s.to_string(), args));
//         return Ok(expression);
//     }
//     Err("444")
// }
//
// impl Iterator for CodeChunk {
//     type Item = Form;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let mut token_iter = self.tokens.iter();
//         let next_token = token_iter.next()?;
//
//         let form = match next_token {
//             Tokens::Bounds(opening) => {
//                 token_iter.advance_back_by(1).ok()?;
//                 let tt = token_iter.by_ref().cloned().collect();
//                 debug!("Tokens between (): {:?}", tt);
//                 ast(tt)
//             }
//             Tokens::Literal(p) => Some(p.clone().into()),
//             _ => None,
//         };
//         let remaing_tokens = token_iter.cloned().collect();
//         debug!("Remaining tokens: {:?}", remaing_tokens);
//
//         self.tokens = remaing_tokens;
//         form
//     }
// }
