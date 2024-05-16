mod utils;

use std::fmt::Display;
use std::ops::Deref;
use utils::*;

use errify::errify_with;

#[test]
fn simple_closure() {
    #[errify_with(|| format!("closure {arg}"))]
    fn func(arg: i32) -> Result<i32, ErrorWithContext> {
        Err(ErrorWithContext::new(arg))
    }

    let err = func(1).unwrap_err();
    assert_eq!(err.msg.deref(), "1");
    assert_eq!(err.cx.as_deref(), Some("closure 1"));
}

#[test]
fn simple_fn() {
    fn context() -> impl Display {
        ContextExpr::new(2)
    }

    #[errify_with(context)]
    fn func(arg: i32) -> Result<i32, ErrorWithContext> {
        Err(ErrorWithContext::new(arg))
    }

    let err = func(1).unwrap_err();
    assert_eq!(err.msg.deref(), "1");
    assert_eq!(err.cx.as_deref(), Some("ContextExpr(2)"));
}

#[tokio::test]
async fn async_closure() {
    #[errify_with(|| format!("closure {arg}"))]
    async fn func(arg: i32) -> Result<i32, ErrorWithContext> {
        Err(ErrorWithContext::new(arg))
    }

    let err = func(1).await.unwrap_err();
    assert_eq!(err.msg.deref(), "1");
    assert_eq!(err.cx.as_deref(), Some("closure 1"));
}

#[tokio::test]
async fn async_fn() {
    fn context() -> impl Display {
        ContextExpr::new(2)
    }

    #[errify_with(context)]
    async fn func(arg: i32) -> Result<i32, ErrorWithContext> {
        Err(ErrorWithContext::new(arg))
    }

    let err = func(1).await.unwrap_err();
    assert_eq!(err.msg.deref(), "1");
    assert_eq!(err.cx.as_deref(), Some("ContextExpr(2)"));
}

#[test]
fn unsafe_closure() {
    #[errify_with(|| format!("closure {arg}"))]
    unsafe fn func(arg: i32) -> Result<i32, ErrorWithContext> {
        Err(ErrorWithContext::new(arg))
    }

    let err = unsafe { func(1).unwrap_err() };
    assert_eq!(err.msg.deref(), "1");
    assert_eq!(err.cx.as_deref(), Some("closure 1"));
}

#[test]
fn unsafe_fn() {
    fn context() -> impl Display {
        ContextExpr::new(2)
    }

    #[errify_with(context)]
    unsafe fn func(arg: i32) -> Result<i32, ErrorWithContext> {
        Err(ErrorWithContext::new(arg))
    }

    let err = unsafe { func(1).unwrap_err() };
    assert_eq!(err.msg.deref(), "1");
    assert_eq!(err.cx.as_deref(), Some("ContextExpr(2)"));
}

#[tokio::test]
async fn async_unsafe_closure() {
    #[errify_with(|| format!("closure {arg}"))]
    async unsafe fn func(arg: i32) -> Result<i32, ErrorWithContext> {
        Err(ErrorWithContext::new(arg))
    }

    let err = unsafe { func(1).await.unwrap_err() };
    assert_eq!(err.msg.deref(), "1");
    assert_eq!(err.cx.as_deref(), Some("closure 1"));
}

#[tokio::test]
async fn async_unsafe_fn() {
    fn context() -> impl Display {
        ContextExpr::new(2)
    }

    #[errify_with(context)]
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
        #[errify_with(|| format!("closure self = {self:?}"))]
        fn func(&self, arg: String) -> Result<i32, ErrorWithContext> {
            Err(ErrorWithContext::new(arg))
        }
    }

    let err = Struct.func("argument".to_owned()).unwrap_err();
    assert_eq!(err.msg.deref(), "argument");
    assert_eq!(err.cx.as_deref(), Some("closure self = Struct"));
}

#[test]
fn trait_method() {
    trait Trait {
        fn func(&self, arg: String) -> Result<i32, ErrorWithContext>;
    }

    #[derive(Debug)]
    struct Struct;

    impl Trait for Struct {
        #[errify_with(|| format!("closure self = {self:?}"))]
        fn func(&self, arg: String) -> Result<i32, ErrorWithContext> {
            Err(ErrorWithContext::new(arg))
        }
    }

    let err = Trait::func(&Struct, "argument".to_owned()).unwrap_err();
    assert_eq!(err.msg.deref(), "argument");
    assert_eq!(err.cx.as_deref(), Some("closure self = Struct"));
}

#[test]
fn check_visibility() {
    pub mod multiple {
        use super::*;
        pub mod module {
            use super::*;
            #[errify_with(|| format!("closure {arg}"))]
            pub fn func(arg: i32) -> Result<i32, ErrorWithContext> {
                Err(ErrorWithContext::new(arg))
            }
        }
    }

    let err = multiple::module::func(1).unwrap_err();
    assert_eq!(err.msg.deref(), "1");
    assert_eq!(err.cx.as_deref(), Some("closure 1"));
}

#[cfg(feature = "anyhow")]
#[test]
fn anyhow_error() {
    #[errify_with(|| format!("closure {arg} = {}", arg))]
    fn func(arg: i32) -> Result<i32, anyhow::Error> {
        Err(anyhow::anyhow!("error {}", arg))
    }

    let err = func(1).unwrap_err();
    let context_err = err.to_string();
    let custom_err = err.root_cause().to_string();
    assert_eq!(context_err, "closure 1 = 1");
    assert_eq!(custom_err, "error 1");
}

#[cfg(feature = "eyre")]
#[test]
fn eyre_error() {
    #[errify_with(|| format!("closure {arg} = {}", arg))]
    fn func(arg: i32) -> Result<i32, eyre::Report> {
        Err(eyre::eyre!("error {}", arg))
    }

    let err = func(1).unwrap_err();
    let context_err = err.to_string();
    let custom_err = err.root_cause().to_string();
    assert_eq!(context_err, "closure 1 = 1");
    assert_eq!(custom_err, "error 1");
}
