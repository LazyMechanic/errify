mod utils;

use std::ops::Deref;

use errify::errify;
use utils::*;

#[test]
fn literal_position_arg() {
    #[errify("literal {arg} = {}", arg)]
    fn func(arg: i32) -> Result<i32, ErrorWithContext> {
        Err(ErrorWithContext::new(arg))
    }

    let err = func(1).unwrap_err();
    assert_eq!(err.msg.deref(), "1");
    assert_eq!(err.cx.as_deref(), Some("literal 1 = 1"));
}

#[test]
fn simple_literal() {
    #[errify("literal {arg}")]
    fn func(arg: i32) -> Result<i32, ErrorWithContext> {
        Err(ErrorWithContext::new(arg))
    }

    let err = func(1).unwrap_err();
    assert_eq!(err.msg.deref(), "1");
    assert_eq!(err.cx.as_deref(), Some("literal 1"));
}

#[test]
fn simple_expr() {
    #[errify(ContextExpr::new(2))]
    fn func(arg: i32) -> Result<i32, ErrorWithContext> {
        Err(ErrorWithContext::new(arg))
    }

    let err = func(1).unwrap_err();
    assert_eq!(err.msg.deref(), "1");
    assert_eq!(err.cx.as_deref(), Some("ContextExpr(2)"));
}

#[tokio::test]
async fn async_literal() {
    #[errify("literal {arg}")]
    async fn func(arg: i32) -> Result<i32, ErrorWithContext> {
        Err(ErrorWithContext::new(arg))
    }

    let err = func(1).await.unwrap_err();
    assert_eq!(err.msg.deref(), "1");
    assert_eq!(err.cx.as_deref(), Some("literal 1"));
}

#[tokio::test]
async fn async_expr() {
    #[errify(ContextExpr::new(2))]
    async fn func(arg: i32) -> Result<i32, ErrorWithContext> {
        Err(ErrorWithContext::new(arg))
    }

    let err = func(1).await.unwrap_err();
    assert_eq!(err.msg.deref(), "1");
    assert_eq!(err.cx.as_deref(), Some("ContextExpr(2)"));
}

#[test]
fn unsafe_literal() {
    #[errify("literal {arg}")]
    unsafe fn func(arg: i32) -> Result<i32, ErrorWithContext> {
        Err(ErrorWithContext::new(arg))
    }

    let err = unsafe { func(1).unwrap_err() };
    assert_eq!(err.msg.deref(), "1");
    assert_eq!(err.cx.as_deref(), Some("literal 1"));
}

#[test]
fn unsafe_expr() {
    #[errify(ContextExpr::new(2))]
    unsafe fn func(arg: i32) -> Result<i32, ErrorWithContext> {
        Err(ErrorWithContext::new(arg))
    }

    let err = unsafe { func(1).unwrap_err() };
    assert_eq!(err.msg.deref(), "1");
    assert_eq!(err.cx.as_deref(), Some("ContextExpr(2)"));
}

#[tokio::test]
async fn async_unsafe_literal() {
    #[errify("literal {arg}")]
    async unsafe fn func(arg: i32) -> Result<i32, ErrorWithContext> {
        Err(ErrorWithContext::new(arg))
    }

    let err = unsafe { func(1).await.unwrap_err() };
    assert_eq!(err.msg.deref(), "1");
    assert_eq!(err.cx.as_deref(), Some("literal 1"));
}

#[tokio::test]
async fn async_unsafe_expr() {
    #[errify(ContextExpr::new(2))]
    async unsafe fn func(arg: i32) -> Result<i32, ErrorWithContext> {
        Err(ErrorWithContext::new(arg))
    }

    let err = unsafe { func(1).await.unwrap_err() };
    assert_eq!(err.msg.deref(), "1");
    assert_eq!(err.cx.as_deref(), Some("ContextExpr(2)"));
}

#[test]
fn method() {
    #[derive(Debug)]
    struct Struct;

    impl Struct {
        #[errify("literal self = {self:?}, arg = {}", arg)]
        fn func(&self, arg: String) -> Result<i32, ErrorWithContext> {
            Err(ErrorWithContext::new(arg))
        }
    }

    let err = Struct.func("argument".to_owned()).unwrap_err();
    assert_eq!(err.msg.deref(), "argument");
    assert_eq!(
        err.cx.as_deref(),
        Some("literal self = Struct, arg = argument")
    );
}

#[test]
fn trait_method() {
    trait Trait {
        fn func(&self, arg: String) -> Result<i32, ErrorWithContext>;
    }

    #[derive(Debug)]
    struct Struct;

    impl Trait for Struct {
        #[errify("literal self = {self:?}, arg = {}", arg)]
        fn func(&self, arg: String) -> Result<i32, ErrorWithContext> {
            Err(ErrorWithContext::new(arg))
        }
    }

    let err = Trait::func(&Struct, "argument".to_owned()).unwrap_err();
    assert_eq!(err.msg.deref(), "argument");
    assert_eq!(
        err.cx.as_deref(),
        Some("literal self = Struct, arg = argument")
    );
}

#[test]
fn check_visibility() {
    pub mod multiple {
        use super::*;
        pub mod module {
            use super::*;
            #[errify("literal {arg}")]
            pub fn func(arg: i32) -> Result<i32, ErrorWithContext> {
                Err(ErrorWithContext::new(arg))
            }
        }
    }

    let err = multiple::module::func(1).unwrap_err();
    assert_eq!(err.msg.deref(), "1");
    assert_eq!(err.cx.as_deref(), Some("literal 1"));
}

#[cfg(feature = "anyhow")]
#[test]
fn anyhow_error() {
    #[errify("literal {arg} = {}", arg)]
    fn func(arg: i32) -> Result<i32, anyhow::Error> {
        Err(anyhow::anyhow!("error {}", arg))
    }

    let err = func(1).unwrap_err();
    let context_err = err.to_string();
    let custom_err = err.root_cause().to_string();
    assert_eq!(context_err, "literal 1 = 1");
    assert_eq!(custom_err, "error 1");
}

#[cfg(feature = "eyre")]
#[test]
fn eyre_error() {
    #[errify("literal {arg} = {}", arg)]
    fn func(arg: i32) -> Result<i32, eyre::Report> {
        Err(eyre::eyre!("error {}", arg))
    }

    let err = func(1).unwrap_err();
    let context_err = err.to_string();
    let custom_err = err.root_cause().to_string();
    assert_eq!(context_err, "literal 1 = 1");
    assert_eq!(custom_err, "error 1");
}
