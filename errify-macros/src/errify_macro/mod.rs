mod input;
mod output;

use proc_macro2::TokenStream;
use proc_macro2_diagnostics::Diagnostic;
use quote::{quote, ToTokens};

use crate::errify_macro::{
    input::{ErrifyMacroArgs, ErrifyWithMacroArgs, Input},
    output::Output,
};

pub fn errify_impl(args: TokenStream, input: TokenStream) -> Result<TokenStream, Diagnostic> {
    let m = ErrifyMacro::from_ast(args, input)?;
    let res = quote! { #m };

    Ok(res)
}

struct ErrifyMacro {
    output: Output,
}

impl ErrifyMacro {
    pub fn from_ast(args: TokenStream, input: TokenStream) -> syn::Result<Self> {
        let args = syn::parse2::<ErrifyMacroArgs>(args)?;
        let input = syn::parse2::<Input>(input)?;

        let output = Output::from_ast(args.into(), input)?;
        Ok(Self { output })
    }
}

impl ToTokens for ErrifyMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.output.to_tokens(tokens)
    }
}

pub fn errify_with_impl(args: TokenStream, input: TokenStream) -> Result<TokenStream, Diagnostic> {
    let m = ErrifyWithMacro::from_ast(args, input)?;
    let res = quote! { #m };

    Ok(res)
}

struct ErrifyWithMacro {
    output: Output,
}

impl ErrifyWithMacro {
    pub fn from_ast(args: TokenStream, input: TokenStream) -> syn::Result<Self> {
        let args = syn::parse2::<ErrifyWithMacroArgs>(args)?;
        let input = syn::parse2::<Input>(input)?;

        let output = Output::from_ast(args.into(), input)?;
        Ok(Self { output })
    }
}

impl ToTokens for ErrifyWithMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.output.to_tokens(tokens)
    }
}
