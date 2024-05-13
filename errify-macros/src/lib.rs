mod errify_macro;
mod utils;

use proc_macro::TokenStream;

use crate::errify_macro::{errify_impl, errify_with_impl};

/// Macro that provides error context on entire function.
/// Supports `async` functions.
///
/// Constraint is `T: Display + Send + Sync + 'static`.
///
/// # Syntax
/// ```text
/// #[errify( $($err_ty:ty),? $( $fmt:literal $(, $arg:expr)* ) | $expr:expr )]
/// ```
///
/// # Usage example
///
/// ### Format string with arguments
/// ```ignore
/// use errify::errify;
///
/// #[errify("Custom error context, with argument capturing {arg} = {}", arg)]
/// fn func(arg: i32) -> Result<(), CustomError> {
///     // ...
/// }
/// ```
///
/// ### Expression
/// ```ignore
/// use errify::errify;
///
/// #[errify(String::from("Custom error context"))]
/// fn func(arg: i32) -> Result<(), CustomError> {
///     // ...
/// }
/// ```
///
/// ### Custom error type
/// ```ignore
/// use errify::{errify, WrapErr};
///
/// struct CustomError { ... }
/// impl WrapErr<CustomError> for CustomError { ... }
///
/// #[errify(CustomError, "Custom error context")]
/// fn func(arg: i32) -> Result<(), CustomError> {
///     // ...
/// }
/// ```
#[proc_macro_attribute]
pub fn errify(args: TokenStream, input: TokenStream) -> TokenStream {
    match errify_impl(args.into(), input.into()) {
        Ok(tokens) => tokens.into(),
        Err(diag) => diag.emit_as_expr_tokens().into(),
    }
}

/// Macro that provides lazy error context on entire function.
/// Supports `async` functions.
///
/// Constraint is `F: FnOnce() -> impl Display + Send + Sync + 'static`.
///
/// # Syntax
/// ```text
/// #[errify_with( $($err_ty:ty),? $closure:expr | $func:ident )]
/// ```
///
/// # Usage example
///
/// ### Closure
/// ```ignore
/// use errify::errify_with;
///
/// #[errify_with(|| "Custom error context from closure")]
/// fn func(arg: i32) -> Result<(), CustomError> {
///     // ...
/// }
/// ```
///
/// ### Function
/// ```ignore
/// use errify::errify_with;
///
/// fn context_provider() -> impl Display { "Context from function" }
///
/// #[errify_with(context_provider)]
/// fn func(arg: i32) -> Result<(), CustomError> {
///     // ...
/// }
/// ```
///
/// ### Custom error type
/// ```ignore
/// use errify::{errify_with, WrapErr};
///
/// struct CustomError { ... }
/// impl WrapErr<CustomError> for CustomError { ... }
///
/// #[errify_with(CustomError, || "Custom error context")]
/// fn func(arg: i32) -> Result<(), CustomError> {
///     // ...
/// }
/// ```
#[proc_macro_attribute]
pub fn errify_with(args: TokenStream, input: TokenStream) -> TokenStream {
    match errify_with_impl(args.into(), input.into()) {
        Ok(tokens) => tokens.into(),
        Err(diag) => diag.emit_as_expr_tokens().into(),
    }
}
