mod context_macro;
mod context_provider;
mod error_provider;
mod utils;

use proc_macro::TokenStream;

use crate::context_macro::{context_impl, with_context_impl};

#[proc_macro_attribute]
pub fn context(args: TokenStream, input: TokenStream) -> TokenStream {
    match context_impl(args.into(), input.into()) {
        Ok(tokens) => tokens.into(),
        Err(diag) => diag.emit_as_expr_tokens().into(),
    }
}

#[proc_macro_attribute]
pub fn with_context(args: TokenStream, input: TokenStream) -> TokenStream {
    match with_context_impl(args.into(), input.into()) {
        Ok(tokens) => tokens.into(),
        Err(diag) => diag.emit_as_expr_tokens().into(),
    }
}
