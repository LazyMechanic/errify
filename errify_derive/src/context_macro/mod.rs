mod input;
mod output;

use proc_macro2::TokenStream;
use proc_macro2_diagnostics::Diagnostic;
use quote::{quote, ToTokens};

use crate::context_macro::{
    input::{ContextArgs, Input, WithContextArgs},
    output::Output,
};

pub fn context_impl(args: TokenStream, input: TokenStream) -> Result<TokenStream, Diagnostic> {
    let m = ContextMacro::parse(args, input)?;
    let res = quote! { #m };

    Ok(res)
}

struct ContextMacro {
    output: Output,
}

impl ContextMacro {
    pub fn parse(args: TokenStream, input: TokenStream) -> syn::Result<Self> {
        let args = syn::parse2::<ContextArgs>(args)?;
        let input = syn::parse2::<Input>(input)?;

        let output = Output::parse(args.into(), input)?;
        Ok(Self { output })
    }
}

impl ToTokens for ContextMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.output.to_tokens(tokens)
    }
}

pub fn with_context_impl(args: TokenStream, input: TokenStream) -> Result<TokenStream, Diagnostic> {
    let m = WithContextMacro::parse(args, input)?;
    let res = quote! { #m };

    Ok(res)
}

struct WithContextMacro {
    output: Output,
}

impl WithContextMacro {
    pub fn parse(args: TokenStream, input: TokenStream) -> syn::Result<Self> {
        let args = syn::parse2::<WithContextArgs>(args)?;
        let input = syn::parse2::<Input>(input)?;

        let output = Output::parse(args.into(), input)?;
        Ok(Self { output })
    }
}

impl ToTokens for WithContextMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.output.to_tokens(tokens)
    }
}
