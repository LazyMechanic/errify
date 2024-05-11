mod context_macro;
mod context_provider;
mod error_provider;
mod utils;

use proc_macro::TokenStream;

use crate::context_macro::{context_impl, with_context_impl};

/// Macro that provides error context on entire function.
/// Supports `async` and `unsafe` functions.
///
/// # Usage example
///
/// ### Format string with arguments
/// ```ignore
/// #[errify::context("Custom error context, with argument capturing {arg} = {}", arg)]
/// fn func(arg: i32) -> Result<(), CustomError> {
///     // ...
/// }
/// ```
///
/// ### Expression
/// ```ignore
/// #[errify::context(String::from("Custom error context"))]
/// fn func(arg: i32) -> Result<(), CustomError> {
///     // ...
/// }
/// ```
/// Constraint is `T: Display + Send + Sync + 'static`.
///
#[proc_macro_attribute]
pub fn context(args: TokenStream, input: TokenStream) -> TokenStream {
    match context_impl(args.into(), input.into()) {
        Ok(tokens) => tokens.into(),
        Err(diag) => diag.emit_as_expr_tokens().into(),
    }
}

/// Macro that provides lazy error context on entire function.
/// Supports `async` and `unsafe` functions.
///
/// # Usage example
///
/// ### Closure
/// ```ignore
/// #[errify::with_context(|| "Custom error context from closure")]
/// fn func(arg: i32) -> Result<(), CustomError> {
///     // ...
/// }
/// ```
///
/// ### Function
/// ```ignore
/// fn context_provider() -> impl Display { "Context from function" }
///
/// #[errify::with_context(context_provider)]
/// fn func(arg: i32) -> Result<(), CustomError> {
///     // ...
/// }
/// ```
///
/// Constraint is `F: FnOnce() -> impl Display + Send + Sync + 'static`.
#[proc_macro_attribute]
pub fn with_context(args: TokenStream, input: TokenStream) -> TokenStream {
    match with_context_impl(args.into(), input.into()) {
        Ok(tokens) => tokens.into(),
        Err(diag) => diag.emit_as_expr_tokens().into(),
    }
}
