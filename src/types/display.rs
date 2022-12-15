use crate::types::*;

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bounds(b) => write!(f, "{}", b),
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
            Self::LeftBracket => write!(f, "["),
            Self::RightBracket => write!(f, "]"),
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
            Form::List(forms) => {
                let forms_string: Vec<String> = forms.iter().map(|f| f.to_string()).collect();
                write!(f, "[{}]", forms_string.join(" "))
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
            Literal::List(s) => {
                let literal_strings: Vec<String> = s.iter().map(|p| p.to_string()).collect();
                write!(f, "[{}]", literal_strings.join(" "))
            }
        }
    }
}
impl fmt::Display for RuntimeObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeObject::Primitive(p) => write!(f, "{}", p),
            RuntimeObject::Function(f) => todo!(),
        }
    }
}
