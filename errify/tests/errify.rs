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
struct CustomErrorWithContext {
    err: std::io::Error,
    cx: String,
}

impl Display for CustomErrorWithContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.cx)
    }
}

impl Error for CustomErrorWithContext {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.err)
    }
}

impl<E: Error + 'static> WrapErr<E> for CustomErrorWithContext {
    fn wrap_err<C>(err: E, context: C) -> Self
    where
        C: Display + Send + Sync + 'static,
    {
        Self {
            err: std::io::Error::new(std::io::ErrorKind::Other, err.to_string()),
            cx: context.to_string(),
        }
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

#[test]
fn custom_error_literal() {
    #[errify(CustomErrorWithContext, "literal {arg} = {}", arg)]
    fn test(arg: i32) -> Result<i32, CustomError> {
        Err(CustomError(arg))
    }

    let err = test(1).unwrap_err();
    let context_err = err.to_string();
    let custom_err = err.err.to_string();
    assert_eq!(context_err, "literal 1 = 1");
    assert_eq!(custom_err, "CustomError(1)");
}

#[test]
fn custom_error_expr() {
    #[errify(CustomErrorWithContext, CustomError::new(2))]
    fn test(arg: i32) -> Result<i32, CustomError> {
        Err(CustomError(arg))
    }

    let err = test(1).unwrap_err();
    let context_err = err.to_string();
    let custom_err = err.err.to_string();
    assert_eq!(context_err, "CustomError(2)");
    assert_eq!(custom_err, "CustomError(1)");
}

#[test]
fn method() {
    #[derive(Debug)]
    struct Struct;

    impl Struct {
        #[errify("literal self = {self:?}, arg = {}", arg)]
        fn func(&self, arg: String) -> Result<i32, CustomError> {
            Err(CustomError(0))
        }
    }

    let err = Struct.func("argument".to_owned()).unwrap_err();
    let context_err = err.to_string();
    let custom_err = err.root_cause().to_string();
    assert_eq!(context_err, "literal self = Struct, arg = argument");
    assert_eq!(custom_err, "CustomError(0)");
}

#[test]
fn trait_method() {
    #[derive(Debug)]
    struct TraitError(Option<String>);
    impl WrapErr<TraitError> for TraitError {
        fn wrap_err<C>(mut err: TraitError, context: C) -> Self
        where
            C: Display + Send + Sync + 'static,
        {
            err.0 = Some(context.to_string());
            err
        }
    }

    trait Trait {
        fn func(&self, arg: String) -> Result<i32, TraitError>;
    }

    #[derive(Debug)]
    struct Struct;

    impl Trait for Struct {
        #[errify(TraitError, "literal self = {self:?}, arg = {}", arg)]
        fn func(&self, arg: String) -> Result<i32, TraitError> {
            Err(TraitError(None))
        }
    }

    let err = Trait::func(&Struct, "argument".to_owned()).unwrap_err();
    let context_err = format!("{err:?}");
    assert_eq!(
        context_err,
        r#"TraitError(Some("literal self = Struct, arg = argument"))"#
    );
}

#[test]
fn check_visibility() {
    pub mod multiple {
        use super::*;
        pub mod module {
            use super::*;
            #[errify("literal")]
            pub fn test(arg: i32) -> Result<i32, CustomError> {
                Err(CustomError(arg))
            }
        }
    }

    let err = multiple::module::test(1).unwrap_err();
    let context_err = err.to_string();
    let custom_err = err.root_cause().to_string();
    assert_eq!(context_err, "literal");
    assert_eq!(custom_err, "CustomError(1)");
}

#[cfg(feature = "anyhow")]
#[test]
fn anyhow_error() {
    #[errify(anyhow::Error, "literal {arg} = {}", arg)]
    fn test(arg: i32) -> Result<i32, anyhow::Error> {
        Err(anyhow::anyhow!("error {}", arg))
    }

    let err = test(1).unwrap_err();
    let context_err = err.to_string();
    let custom_err = err.root_cause().to_string();
    assert_eq!(context_err, "literal 1 = 1");
    assert_eq!(custom_err, "error 1");

    #[cfg(feature = "eyre")]
    {
        #[errify(anyhow::Error, "literal {arg} = {}", arg)]
        fn test2(arg: i32) -> Result<i32, eyre::Report> {
            Err(eyre::eyre!("error {}", arg))
        }

        let err = test2(1).unwrap_err();
        let context_err = err.to_string();
        let custom_err = err.root_cause().to_string();
        assert_eq!(context_err, "literal 1 = 1");
        assert_eq!(custom_err, "error 1");
    }
}

#[cfg(feature = "eyre")]
#[test]
fn eyre_error() {
    #[errify(eyre::Report, "literal {arg} = {}", arg)]
    fn test(arg: i32) -> Result<i32, eyre::Report> {
        Err(eyre::eyre!("error {}", arg))
    }

    let err = test(1).unwrap_err();
    let context_err = err.to_string();
    let custom_err = err.root_cause().to_string();
    assert_eq!(context_err, "literal 1 = 1");
    assert_eq!(custom_err, "error 1");

    #[cfg(feature = "anyhow")]
    {
        #[errify(eyre::Report, "literal {arg} = {}", arg)]
        fn test2(arg: i32) -> Result<i32, anyhow::Error> {
            Err(anyhow::anyhow!("error {}", arg))
        }

        let err = test2(1).unwrap_err();
        let context_err = err.to_string();
        let custom_err = err.root_cause().to_string();
        assert_eq!(context_err, "literal 1 = 1");
        assert_eq!(custom_err, "error 1");
    }
}
