use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use errify::{errify, WrapErr};

#[derive(Debug)]
struct CustomError(i32);

impl CustomError {
    pub fn new(i: i32) -> Self {
        Self(i)
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("CustomError({})", self.0))
    }
}

impl Error for CustomError {}

#[derive(Debug)]
struct CustomErrorWithContext<E> {
    err: E,
    cx: String,
}

impl<E> Display for CustomErrorWithContext<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.cx)
    }
}

impl<E: Error + 'static> Error for CustomErrorWithContext<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.err)
    }
}

impl<E> WrapErr<E> for CustomErrorWithContext<E> {
    fn wrap_err<C>(err: E, context: C) -> Self where C: Display + Send + Sync + 'static {
        Self{ err, cx: context.to_string() }
    }
}

#[test]
fn literal_position_arg() {
    #[errify("literal {arg} = {}", arg)]
    fn test(arg: i32) -> Result<i32, CustomError> {
        Err(CustomError(arg))
    }

    let err = test(1).unwrap_err();
    let context_err = err.to_string();
    let custom_err = err.root_cause().to_string();
    assert_eq!(context_err, "literal 1 = 1");
    assert_eq!(custom_err, "CustomError(1)");
}

#[test]
fn simple_literal() {
    #[errify("literal {arg}")]
    fn test(arg: i32) -> Result<i32, CustomError> {
        Err(CustomError(arg))
    }

    let err = test(1).unwrap_err();
    let context_err = err.to_string();
    let custom_err = err.root_cause().to_string();
    assert_eq!(context_err, "literal 1");
    assert_eq!(custom_err, "CustomError(1)");
}

#[test]
fn simple_expr() {
    #[errify(CustomError::new(2))]
    fn test(arg: i32) -> Result<i32, CustomError> {
        Err(CustomError(arg))
    }

    let err = test(1).unwrap_err();
    let context_err = err.to_string();
    let custom_err = err.root_cause().to_string();
    assert_eq!(context_err, "CustomError(2)");
    assert_eq!(custom_err, "CustomError(1)");
}

#[tokio::test]
async fn async_literal() {
    #[errify("literal {arg}")]
    async fn test(arg: i32) -> Result<i32, CustomError> {
        Err(CustomError(arg))
    }

    let err = test(1).await.unwrap_err();
    let context_err = err.to_string();
    let custom_err = err.root_cause().to_string();
    assert_eq!(context_err, "literal 1");
    assert_eq!(custom_err, "CustomError(1)");
}

#[tokio::test]
async fn async_expr() {
    #[errify(CustomError::new(2))]
    async fn test(arg: i32) -> Result<i32, CustomError> {
        Err(CustomError(arg))
    }

    let err = test(1).await.unwrap_err();
    let context_err = err.to_string();
    let custom_err = err.root_cause().to_string();
    assert_eq!(context_err, "CustomError(2)");
    assert_eq!(custom_err, "CustomError(1)");
}

#[test]
fn unsafe_literal() {
    #[errify("literal {arg}")]
    unsafe fn test(arg: i32) -> Result<i32, CustomError> {
        Err(CustomError(arg))
    }

    let err = unsafe { test(1).unwrap_err() };
    let context_err = err.to_string();
    let custom_err = err.root_cause().to_string();
    assert_eq!(context_err, "literal 1");
    assert_eq!(custom_err, "CustomError(1)");
}

#[test]
fn unsafe_expr() {
    #[errify(CustomError::new(2))]
    unsafe fn test(arg: i32) -> Result<i32, CustomError> {
        Err(CustomError(arg))
    }

    let err = unsafe { test(1).unwrap_err() };
    let context_err = err.to_string();
    let custom_err = err.root_cause().to_string();
    assert_eq!(context_err, "CustomError(2)");
    assert_eq!(custom_err, "CustomError(1)");
}

#[tokio::test]
async fn async_unsafe_literal() {
    #[errify("literal {arg}")]
    async unsafe fn test(arg: i32) -> Result<i32, CustomError> {
        Err(CustomError(arg))
    }

    let err = unsafe { test(1).await.unwrap_err() };
    let context_err = err.to_string();
    let custom_err = err.root_cause().to_string();
    assert_eq!(context_err, "literal 1");
    assert_eq!(custom_err, "CustomError(1)");
}

#[tokio::test]
async fn async_unsafe_expr() {
    #[errify(CustomError::new(2))]
    async unsafe fn test(arg: i32) -> Result<i32, CustomError> {
        Err(CustomError(arg))
    }

    let err = unsafe { test(1).await.unwrap_err() };
    let context_err = err.to_string();
    let custom_err = err.root_cause().to_string();
    assert_eq!(context_err, "CustomError(2)");
    assert_eq!(custom_err, "CustomError(1)");
}
