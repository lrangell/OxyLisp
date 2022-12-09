use crate::types::*;

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bounds(TokenBounds::LeftParen) => write!(f, "("),
            Self::Bounds(TokenBounds::RightParen) => write!(f, ")"),
            Self::Literal(p) => write!(f, "{}", p),
            Self::Symbol(p) => write!(f, "{}", p),
        }
    }
}
impl fmt::Display for TokenBounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LeftParen => write!(f, "("),
            Self::RightParen => write!(f, ")"),
        }
    }
}

impl fmt::Display for Form {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Form::Literal(p) => write!(f, "{}", p),
            Form::Symbol(s) => write!(f, "{}", s),
            Form::CallExpression((to_call, forms)) => {
                let forms_string: Vec<String> = forms.iter().map(|f| f.to_string()).collect();
                write!(f, "({} {})", to_call, forms_string.join(" "))
            }
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{s}"),
            Literal::Integer(s) => write!(f, "{s}"),
            Literal::Bool(s) => write!(f, "{s}"),
        }
    }
}
