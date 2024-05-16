use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    ops::Deref,
};

use errify::WrapErr;

#[derive(Debug)]
pub struct ContextExpr(i32);

impl ContextExpr {
    pub fn new(i: i32) -> Self {
        Self(i)
    }
}

impl Display for ContextExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("ContextExpr({})", self.0))
    }
}

impl Error for ContextExpr {}

#[derive(Debug)]
pub struct StringError(pub String);

impl Display for StringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for StringError {}

impl Deref for StringError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for StringError {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub struct ErrorWithContext {
    pub msg: StringError,
    pub cx: Option<String>,
}

impl ErrorWithContext {
    pub fn new(msg: impl Display) -> Self {
        Self {
            msg: format!("{msg}").into(),
            cx: None,
        }
    }
}

impl Display for ErrorWithContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.cx {
            None => write!(f, "{}", self.msg),
            Some(cx) => write!(f, "{cx}"),
        }
    }
}

impl Error for ErrorWithContext {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.msg)
    }
}

impl WrapErr for ErrorWithContext {
    fn wrap_err<C>(self, context: C) -> Self
    where
        C: Display + Send + Sync + 'static,
    {
        Self {
            msg: self.msg,
            cx: Some(context.to_string()),
        }
    }
}
