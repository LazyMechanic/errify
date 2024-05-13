//! # errify
//!
//! This library provides the macros that provide error context for the entire function
//! via [`anyhow`] and [`eyre`] crates.
//!
//! ## Features
//! - `anyhow` *(enabled by default)*: Enables error and context providers via the [`anyhow`] crate
//! - `eyre`: Enables error and context providers via the [`eyre`] crate
//!
//! Simultaneously enabling both features when using the [`errify`], or [`errify_with`] macros
//! will result in a compilation error if the error type is not specified [explicitly](#specify-error-type-explicitly).
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
//! use errify::errify;
//!
//! #[errify("Custom error context, with argument capturing {arg} = {}", arg)]
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
//! #     impl<E> errify::WrapErr<E> for Error
//! #         where
//! #             E: std::error::Error + Send + Sync + 'static,
//! #     {
//! #         fn wrap_err<C>(err: E, context: C) -> Self
//! #             where
//! #                 C: std::fmt::Display + Send + Sync + 'static,
//! #         {
//! #             unimplemented!()
//! #         }
//! #     }
//! # }
//! fn func(arg: i32) -> Result<(), anyhow::Error> {
//!     let cx = std::borrow::Cow::<'static, str>::Owned(format!("Custom error context, with argument capturing {arg} = {}", arg));
//!     let res = {
//!         let f = move || {
//!             // ...
//!             # Err(CustomError)
//!         };
//!         // Type inference hack
//!         let f_res: Result<(), CustomError> = (f)();
//!         f_res
//!     };
//!     match res {
//!         Ok(v) => Ok(v),
//!         Err(err) => Err(<anyhow::Error as errify::WrapErr<_>>::wrap_err(err, cx)),
//!     }
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
//! use errify::errify;
//!
//! #[errify(String::from("Hello context from String"))]
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
//! Note that [`errify`] macro is not lazy, a context will be created
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
//! use errify::errify_with;
//!
//! #[errify_with(|| format!("Wow, context from lambda, and it can also capture arguments {arg}"))]
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
//! Macros also support `async` functions.
//!
//! ### Specify error type explicitly
//!
//! If you don't want to use `anyhow::Error` / `eyre::Report` and want to use your own error type,
//! there is also a solution:
//!
//! ```
//! # use errify::errify;
//! # #[derive(Debug, Eq, PartialEq)]
//! # struct CustomError;
//! # impl std::fmt::Display for CustomError {
//! #     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//! #         f.write_fmt(format_args!("{self:?}"))
//! #     }
//! # }
//! # impl std::error::Error for CustomError {}
//! // Create your type
//! struct ErrorWithContext {
//!     cause: CustomError,
//!     cx: String,
//! }
//!
//! // Implement `errify::WrapErr`
//! impl errify::WrapErr<CustomError> for ErrorWithContext {
//!     fn wrap_err<C>(err: CustomError, context: C) -> Self
//!     where
//!         C: std::fmt::Display + Send + Sync + 'static,
//!     {
//!         Self {
//!             cause: err,
//!             cx: context.to_string(),
//!         }
//!     }
//! }
//!
//! // Specify the type as the first macro argument
//! #[errify(ErrorWithContext, "error context")]
//! fn func(arg: i32) -> Result<(), CustomError> {
//!     // ...
//!     # Err(CustomError)
//! }
//!
//! # if let Err(err) = func(123) {
//! #     assert_eq!(err.cx, "error context");
//! #     assert_eq!(err.cause, CustomError);
//! # }
//! ```
//!
//! [`errify_with`] also supports a custom error type.
//!
//! [`anyhow`]: https://docs.rs/anyhow/latest/anyhow/
//! [`eyre`]: https://docs.rs/eyre/latest/eyre/
//! [`errify`]: errify_macros::errify
//! [`errify_with`]: errify_macros::errify_with

extern crate alloc;
extern crate core;

#[macro_use]
mod macros;

use alloc::fmt::Display;
#[cfg(any(feature = "anyhow", feature = "eyre"))]
use std::error::Error as StdError;

pub use errify_macros::{errify, errify_with};

/// Provides the `wrap_err` associated function for the error type.
///
/// Implements for your own type if you want to use your custom error type as an error in macros.
pub trait WrapErr<E> {
    /// Wrap the error value with additional context.
    ///
    /// The function should work similarly to [anyhow::Error::context](`https://docs.rs/anyhow/latest/anyhow/struct.Error.html#method.context`),
    /// except that the type should take care of the `err` itself, without a generalized error type.
    fn wrap_err<C>(err: E, context: C) -> Self
    where
        C: Display + Send + Sync + 'static;
}

#[cfg(feature = "anyhow")]
impl<E> WrapErr<E> for anyhow::Error
where
    E: StdError + Send + Sync + 'static,
{
    fn wrap_err<C>(err: E, context: C) -> Self
    where
        C: Display + Send + Sync + 'static,
    {
        anyhow::Error::from(err).context(context)
    }
}

#[cfg(feature = "eyre")]
impl<E> WrapErr<E> for eyre::Report
where
    E: StdError + Send + Sync + 'static,
{
    fn wrap_err<C>(err: E, context: C) -> Self
    where
        C: Display + Send + Sync + 'static,
    {
        eyre::Report::from(err).wrap_err(context)
    }
}

#[doc(hidden)]
pub mod __private {
    use alloc::fmt;
    #[doc(hidden)]
    pub use alloc::{borrow::Cow, format};
    use core::fmt::Arguments;
    #[doc(hidden)]
    pub use core::{
        format_args,
        result::{
            Result,
            Result::{Err, Ok},
        },
    };

    #[cfg(feature = "anyhow")]
    #[doc(hidden)]
    pub use anyhow;
    #[cfg(feature = "eyre")]
    #[doc(hidden)]
    pub use eyre;

    #[doc(hidden)]
    #[inline]
    pub fn format_err(args: Arguments) -> Cow<'static, str> {
        if let Some(message) = args.as_str() {
            // error!("literal"), can downcast to &'static str
            Cow::Borrowed(message)
        } else {
            // error!("interpolate {var}"), can downcast to String
            Cow::Owned(fmt::format(args))
        }
    }
}
