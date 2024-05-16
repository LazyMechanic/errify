//! # errify
//!
//! This library provides the macros that provide error context for the entire function.
//!
//! ## Features
//! - `anyhow`: Implements [`WrapErr`] trait for [`anyhow::Error`]
//! - `eyre`: Implements [`WrapErr`] trait for [`eyre::Report`]
//!
//! ## Context provider
//! There are two macros [`errify`] and [`errify_with`] that provide immediate and lazy context creation respectively.
//! The error type **must** implement the [`WrapErr`] trait for use in macros.
//!
//! Macros also support `async` functions.
//!
//! ### Immediate context
//!
//! To get started, add the attribute macro to the function for which you want to add error context
//! and implement [`errify::WrapErr`] for your error:
//! ```
//! use errify::errify;
//!
//! struct CustomError {
//!     // ...
//! }
//! impl errify::WrapErr for CustomError {
//!     fn wrap_err<C>(self, context: C) -> Self
//!     where
//!         C: std::fmt::Display + Send + Sync + 'static,
//!     {
//!         // ...
//!         # drop(context);
//!         # self
//!     }
//! }
//!
//! #[errify("Custom error context, with argument capturing {arg} = {}", arg)]
//! fn func(arg: i32) -> Result<(), CustomError> {
//!     // ...
//!     # Err(CustomError{})
//! }
//! ```
//!
//! This code expands into something like this:
//! ```
//! # struct CustomError;
//! # impl errify::WrapErr for CustomError {
//! #     fn wrap_err<C>(self, context: C) -> Self
//! #     where
//! #         C: std::fmt::Display + Send + Sync + 'static,
//! #     {
//! #         drop(context);
//! #         self
//! #     }
//! # }
//! fn func(arg: i32) -> Result<(), CustomError> {
//!     let cx = std::borrow::Cow::<'static, str>::Owned(format!("Custom error context, with argument capturing {arg} = {}", arg));
//!     let res = {
//!         let f = move || {
//!             // ...
//!             # Err(CustomError)
//!         };
//!         let f_res: Result<(), CustomError> = (f)();
//!         f_res
//!     };
//!     match res {
//!         Ok(v) => Ok(v),
//!         Err(err) => Err(errify::WrapErr::wrap_err(err, cx)),
//!     }
//! }
//! ```
//!
//! Note that after desugaring your original function converts into closure and move all arguments into it.
//! This is mean that context is created **before** call this function because of arguments, and
//! it could lead to unnecessary allocation even for the success branch.
//!
//! The context can be either the format string or any expression that fits
//! constraint `T: Display + Send + Sync + 'static`:
//! ```
//! # struct CustomError;
//! # impl errify::WrapErr for CustomError {
//! #     fn wrap_err<C>(self, context: C) -> Self
//! #     where
//! #         C: std::fmt::Display + Send + Sync + 'static,
//! #     {
//! #         drop(context);
//! #         self
//! #     }
//! # }
//! use errify::errify;
//!
//! #[errify(String::from("Hello context from String"))]
//! fn func(arg: i32) -> Result<(), CustomError> {
//!     // ...
//!     # Err(CustomError)
//! }
//! ```
//!
//! ### Lazy context
//!
//! If you need lazy initialization of the context, there is another macro:
//! ```
//! # struct CustomError;
//! # impl errify::WrapErr for CustomError {
//! #     fn wrap_err<C>(self, context: C) -> Self
//! #     where
//! #         C: std::fmt::Display + Send + Sync + 'static,
//! #     {
//! #         drop(context);
//! #         self
//! #     }
//! # }
//! use errify::errify_with;
//!
//! #[errify_with(|| format!("Wow, context from lambda, and it can also capture arguments {arg}"))]
//! fn func(arg: i32) -> Result<(), CustomError> {
//!     // ...
//!     # Err(CustomError)
//! }
//! ```
//!
//! The constraint looks similar `F: FnOnce() -> impl Display + Send + Sync + 'static`.
//!
//! You can use either a lambda or define free function:
//! ```
//! # struct CustomError;
//! # impl errify::WrapErr for CustomError {
//! #     fn wrap_err<C>(self, context: C) -> Self
//! #     where
//! #         C: std::fmt::Display + Send + Sync + 'static,
//! #     {
//! #         drop(context);
//! #         self
//! #     }
//! # }
//! use std::fmt::Display;
//! use errify::errify_with;
//!
//! fn ctx() -> impl Display {
//!     "context from free function"
//! }
//!
//! #[errify_with(ctx)]
//! fn func(arg: i32) -> Result<(), CustomError> {
//!     // ...
//!     # Err(CustomError)
//! }
//! ```
//!
//! [`WrapErr`]: crate::WrapErr
//! [`anyhow`]: https://docs.rs/anyhow/latest/anyhow/
//! [`eyre`]: https://docs.rs/eyre/latest/eyre/
//! [`anyhow::Error`]: https://docs.rs/anyhow/latest/anyhow/struct.Error.html
//! [`eyre::Report`]: https://docs.rs/eyre/latest/eyre/struct.Report.html
//! [`errify`]: errify_macros::errify
//! [`errify_with`]: errify_macros::errify_with

extern crate alloc;
extern crate core;

#[macro_use]
mod macros;

use alloc::fmt::Display;

pub use errify_macros::{errify, errify_with};

/// Provides the `wrap_err` associated function for the error type.
///
/// Implements for your own type if you want to use your custom error type as an error in macros.
pub trait WrapErr {
    /// Wrap the error value with additional context.
    ///
    /// The function should work similarly to [anyhow::Error::context](`https://docs.rs/anyhow/latest/anyhow/struct.Error.html#method.context`).
    fn wrap_err<C>(self, context: C) -> Self
    where
        C: Display + Send + Sync + 'static;
}

#[cfg(feature = "anyhow")]
impl WrapErr for anyhow::Error {
    fn wrap_err<C>(self, context: C) -> Self
    where
        C: Display + Send + Sync + 'static,
    {
        anyhow::Error::context(self, context)
    }
}

#[cfg(feature = "eyre")]
impl WrapErr for eyre::Report {
    fn wrap_err<C>(self, context: C) -> Self
    where
        C: Display + Send + Sync + 'static,
    {
        eyre::Report::wrap_err(self, context)
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
            // format_cx!("literal"), can downcast to &'static str
            Cow::Borrowed(message)
        } else {
            // format_cx!("interpolate {var}"), can downcast to String
            Cow::Owned(fmt::format(args))
        }
    }
}
