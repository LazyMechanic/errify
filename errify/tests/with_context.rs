use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

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

#[test]
fn simple_closure() {
    #[errify::with_context(|| format!("literal {arg}"))]
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
fn simple_fn() {
    fn context() -> impl Display {
        CustomError::new(2)
    }

    #[errify::with_context(context)]
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
async fn async_closure() {
    #[errify::with_context(|| format!("literal {arg}"))]
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
async fn async_fn() {
    fn context() -> impl Display {
        CustomError::new(2)
    }

    #[errify::with_context(context)]
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
    #[errify::with_context(|| format!("literal {arg}"))]
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
fn unsafe_custom_error() {
    fn context() -> impl Display {
        CustomError::new(2)
    }

    #[errify::with_context(context)]
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
    #[errify::with_context(|| format!("literal {arg}"))]
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
async fn async_unsafe_custom_error() {
    fn context() -> impl Display {
        CustomError::new(2)
    }

    #[errify::with_context(context)]
    async unsafe fn test(arg: i32) -> Result<i32, CustomError> {
        Err(CustomError(arg))
    }

    let err = unsafe { test(1).await.unwrap_err() };
    let context_err = err.to_string();
    let custom_err = err.root_cause().to_string();
    assert_eq!(context_err, "CustomError(2)");
    assert_eq!(custom_err, "CustomError(1)");
}