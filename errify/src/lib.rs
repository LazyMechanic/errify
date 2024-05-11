//! # errify
//!
//! This library provides the macros that provide error context for the entire function
//! via [`anyhow`] and [`eyre`] crates.
//!
//! ## Features
//! - `anyhow` *(enabled by default)*: Enables error and context providers via the [`anyhow`] crate
//! - `eyre`: Enables error and context providers via the [`eyre`] crate
//!
//! Simultaneously enabling both features when using the [`context`],
//! or [`with_context`] macros will result in a compilation error.
//!
//! ### Simple context
//!
//! To get started, add the attribute macro to the function for which you want to add error context:
//! ```
//! # #[derive(Debug)]
//! # struct CustomError;
//! # impl std::fmt::Display for CustomError {
//! #     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//! #         f.write_fmt(format_args!("{self:?}"))
//! #     }
//! # }
//! # impl std::error::Error for CustomError {}
//! #[errify::context("Custom error context, with argument capturing {arg} = {}", arg)]
//! fn func(arg: i32) -> Result<(), CustomError> {
//!     // ...
//!     # Err(CustomError)
//! }
//! # let err = func(1).unwrap_err();
//! # let err_context = err.to_string();
//! # let err_custom = err.root_cause().to_string();
//! # assert_eq!(err_context, "Custom error context, with argument capturing 1 = 1");
//! # assert_eq!(err_custom, "CustomError");
//! ```
//!
//! This code expands into something like this:
//! ```
//! # #[derive(Debug)]
//! # struct CustomError;
//! # impl std::fmt::Display for CustomError {
//! #     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//! #         f.write_fmt(format_args!("{self:?}"))
//! #     }
//! # }
//! # impl std::error::Error for CustomError {}
//! # mod anyhow {
//! #     #[derive(Debug)]
//! #     pub struct Error;
//! #     impl std::fmt::Display for Error {
//! #         fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { unimplemented!() }
//! #     }
//! #     impl std::error::Error for Error {}
//! #
//! #     pub trait Context<T>: Sized {
//! #         fn context(self, cx: impl std::fmt::Display) -> Result<T, Error> { unimplemented!() }
//! #     }
//! #     impl<T, E> Context<T> for Result<T, E> {}
//! #
//! #     macro_rules! anyhow {
//! #         ($($tt:tt)*) => { "" }
//! #     }
//! #     pub(crate) use anyhow;
//! # }
//! fn func(arg: i32) -> Result<(), anyhow::Error> {
//!     fn _func(arg: i32) -> Result<(), CustomError> {
//!         // ...
//!         # Err(CustomError)
//!     }
//!     let ctx = anyhow::anyhow!("Custom error context, with argument capturing {arg} = {}", arg);
//!     anyhow::Context::context(_func(arg), ctx)
//! }
//! ```
//!
//! The context can be either the format string or any expression that fits
//! constraint `T: Display + Send + Sync + 'static`:
//! ```
//! # #[derive(Debug)]
//! # struct CustomError;
//! # impl std::fmt::Display for CustomError {
//! #     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//! #         f.write_fmt(format_args!("{self:?}"))
//! #     }
//! # }
//! # impl std::error::Error for CustomError {}
//! #[errify::context(String::from("Hello context from String"))]
//! fn func(arg: i32) -> Result<(), CustomError> {
//!     // ...
//!     # Err(CustomError)
//! }
//! # let err = func(1).unwrap_err();
//! # let err_context = err.to_string();
//! # let err_custom = err.root_cause().to_string();
//! # assert_eq!(err_context, "Hello context from String");
//! # assert_eq!(err_custom, "CustomError");
//! ```
//!
//! Note that `#[errify::context(...)]` macro is not lazy, a context will be created
//! before the function is called.
//!
//! ### Lazy context
//!
//! If you need lazy initialization of the context, there is another macro:
//! ```
//! # #[derive(Debug)]
//! # struct CustomError;
//! # impl std::fmt::Display for CustomError {
//! #     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//! #         f.write_fmt(format_args!("{self:?}"))
//! #     }
//! # }
//! # impl std::error::Error for CustomError {}
//! #[errify::with_context(|| format!("Wow, context from lambda, and it can also capture arguments {arg}"))]
//! fn func(arg: i32) -> Result<(), CustomError> {
//!     // ...
//!     # Err(CustomError)
//! }
//! # let err = func(1).unwrap_err();
//! # let err_context = err.to_string();
//! # let err_custom = err.root_cause().to_string();
//! # assert_eq!(err_context, "Wow, context from lambda, and it can also capture arguments 1");
//! # assert_eq!(err_custom, "CustomError");
//! ```
//!
//! The constraint looks similar `F: FnOnce() -> impl Display + Send + Sync + 'static`.
//!
//! You can use either a lambda or define free function.
//!
//! Macros also support `async` and `unsafe` functions.
//!
//! [`anyhow`]: https://docs.rs/anyhow/latest/anyhow/
//! [`eyre`]: https://docs.rs/eyre/latest/eyre/
//! [`context`]: errify_macros::context
//! [`with_context`]: errify_macros::with_context

pub use errify_macros::{context, with_context};

#[doc(hidden)]
pub mod __private {
    #[cfg(feature = "anyhow")]
    pub use anyhow;
    #[cfg(feature = "eyre")]
    pub use eyre;
}
