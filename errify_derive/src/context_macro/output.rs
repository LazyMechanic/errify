use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens};
use syn::{parse_quote, Block, Expr, ImplItem, ItemFn, ReturnType};

use crate::{
    context_macro::input::{Args, Input},
    context_provider,
    context_provider::ContextData,
    error_provider, utils,
};

// TODO: method and impl
#[allow(dead_code)]
pub enum Output {
    Function(ItemFn),
    Method(ItemFn),
    Impl(ImplItem),
}

impl Output {
    pub fn parse(args: Args, input: Input) -> syn::Result<Self> {
        match input {
            Input::Function(f) => Self::parse_func(args, f),
            Input::Method(f) => Self::parse_method(args, f),
            Input::Impl(i) => Self::parse_impl(args, i),
        }
    }

    fn parse_func(args: Args, input: ItemFn) -> syn::Result<Self> {
        let inner_fn: ItemFn = {
            let constness = &input.sig.constness;
            let asyncness = &input.sig.asyncness;
            let unsafety = &input.sig.unsafety;
            let abi = &input.sig.abi;
            let ident = format_ident!("_{}", input.sig.ident);
            let (generics_impl, _generics_ty, generics_where) = input.sig.generics.split_for_impl();
            let args = &input.sig.inputs;
            let ret = &input.sig.output;
            let block = &input.block;
            parse_quote! {
                #constness #asyncness #unsafety #abi fn #ident #generics_impl ( #args ) #ret #generics_where #block
            }
        };

        let outer_args = utils::clear_inputs(&input.sig.inputs)?;
        let call_expr: Expr = {
            let inner_fn_ident = &inner_fn.sig.ident;
            let call_args = utils::call_inputs(&outer_args);
            let mut expr = parse_quote! { #inner_fn_ident( #call_args ) };
            if inner_fn.sig.asyncness.is_some() {
                expr = parse_quote! { #expr.await };
            }
            if inner_fn.sig.unsafety.is_some() {
                expr = parse_quote! { unsafe { #expr } };
            }
            expr
        };

        let cx_data = match args {
            Args::None { .. } => {
                return Err(syn::Error::new(
                    args.span(),
                    "The macro requires arguments \
                        (literal with positions arguments or custom error) \
                        above the function",
                ))
            }
            Args::Literal { lit, args } => ContextData::Literal { lit, args },
            Args::ErrorType { expr } => ContextData::Expr { expr },
            Args::Closure { def } => ContextData::Closure { def },
            Args::Function { path } => ContextData::Function { path },
        };
        let cx_expr = context_provider::generic(call_expr, cx_data)?;

        let outer_fn: ItemFn = {
            let constness = &input.sig.constness;
            let asyncness = &input.sig.asyncness;
            let unsafety = &input.sig.unsafety;
            let abi = &input.sig.abi;
            let ident = &input.sig.ident;
            let (generics_impl, _generics_ty, generics_where) = input.sig.generics.split_for_impl();
            let ret: ReturnType = {
                let ok = utils::ok_ty(&input.sig.output)?;
                let err = error_provider::generic()?;
                parse_quote! { -> ::core::result::Result<#ok, #err> }
            };
            let block: Block = parse_quote! {
                {
                    #inner_fn
                    #cx_expr
                }
            };
            parse_quote! {
                #constness #asyncness #unsafety #abi fn #ident #generics_impl ( #outer_args ) #ret #generics_where #block
            }
        };

        Ok(Self::Function(outer_fn))
    }

    fn parse_method(_args: Args, _input: ItemFn) -> syn::Result<Self> {
        // TODO: deselfify fn args and use same algorithm as parse_func
        unimplemented!(
            "Using the macro with a method (with the Self argument) is not yet supported"
        )
    }

    fn parse_impl(_args: Args, _input: ImplItem) -> syn::Result<Self> {
        unimplemented!("Using the macro with a impl block is not yet supported")
    }
}

impl ToTokens for Output {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Output::Function(f) => f.to_tokens(tokens),
            Output::Method(f) => f.to_tokens(tokens),
            Output::Impl(i) => i.to_tokens(tokens),
        }
    }
}
