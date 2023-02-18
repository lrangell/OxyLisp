use crate::{
    lexer::tokenize,
    prelude::{display::PrintAST, *},
};
use anyhow::{anyhow, Result};
use trees::{Node, Tree};

pub fn parse<'a>(tokens: &[Tokens], ast: &'a mut Node<Form>) -> Result<&'a mut Node<Form>> {
    if tokens.is_empty() {
        return Ok(ast);
    }
    let (head, tail) = tokens
        .split_first()
        .ok_or(anyhow!("\nEmpty token vector\n"))?;

    return match head {
        Tokens::Bounds(TokenBounds::LeftParen) => {
            let (Tokens::Symbol(sym), right_tokens) = tail
                .split_first()
                .ok_or(anyhow!("Unexpected empty parens"))? else {return Err(anyhow!(" "))};
            parse_into(
                Form::CallExpression(sym.clone()),
                OpenBoundsTracker::parens(),
                right_tokens,
                ast,
            )
        }
        Tokens::Bounds(TokenBounds::LeftBracket) => {
            parse_into(Form::List, OpenBoundsTracker::brackets(), tail, ast)
        }

        Tokens::Bounds(TokenBounds::LeftCurlyBraces) => {
            parse_into(Form::Record, OpenBoundsTracker::braces(), tail, ast)
        }

        Tokens::Bounds(_) => parse(tail, ast),

        token => {
            let form: Form = token.clone().into();
            ast.push_back(Tree::new(form));
            parse(tail, ast)
        }
    };
}

fn parse_into<'a>(
    root_form: Form,
    tracker: OpenBoundsTracker,
    tokens: &[Tokens],
    ast: &'a mut Node<Form>,
) -> Result<&'a mut Node<Form>> {
    let mut root = Tree::new(root_form);
    let (inner, rest) = split_at_bound(tokens, tracker)?;
    parse(inner, &mut root.root_mut())?;
    ast.push_back(root);
    parse_remaining(rest, ast)
}
fn parse_remaining<'a>(rest: &[Tokens], ast: &'a mut Node<Form>) -> Result<&'a mut Node<Form>> {
    let mut empty_tree = Tree::new(Form::Root);
    parse(rest, &mut empty_tree.root_mut())?;
    ast.append(empty_tree.abandon());
    return Ok(ast);
}

fn split_at_bound(
    tokens: &[Tokens],
    mut tracker: OpenBoundsTracker,
) -> Result<(&[Tokens], &[Tokens])> {
    let split_index = tokens
        .iter()
        .position(|t| tracker.track(t))
        .ok_or(anyhow!("Uneven number of bound tokens"))?;
    Ok(tokens.split_at(split_index))
}

struct OpenBoundsTracker {
    opener: TokenBounds,
    closer: TokenBounds,
    count: i32,
}
impl OpenBoundsTracker {
    fn track(&mut self, bound: &Tokens) -> bool {
        self.count += match bound {
            Tokens::Bounds(b) if *b == self.opener => 1,
            Tokens::Bounds(b) if *b == self.closer => -1,
            _ => 0,
        };
        self.count == 0
    }

    fn parens() -> Self {
        OpenBoundsTracker {
            opener: TokenBounds::LeftParen,
            closer: TokenBounds::RightParen,
            count: 1,
        }
    }
    fn brackets() -> Self {
        OpenBoundsTracker {
            opener: TokenBounds::LeftBracket,
            closer: TokenBounds::RightBracket,
            count: 1,
        }
    }
    fn braces() -> Self {
        OpenBoundsTracker {
            opener: TokenBounds::LeftCurlyBraces,
            closer: TokenBounds::RightCurlyBraces,
            count: 1,
        }
    }
}

pub fn parse_string(code: &str) -> Result<Tree<Form>> {
    let mut tree = Tree::new(Form::Root);
    let root = tree.root_mut().get_mut();
    parse(&tokenize(code), root)?;
    Ok(tree)
}
