use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, spanned::Spanned, Block, Expr, ExprClosure, ImplItemFn, ReturnType};

use crate::{
    context_provider,
    context_provider::ContextData,
    errify_macro::input::{Args, Context, ExplicitContext, Input, LazyContext},
    error_provider, utils,
};

pub struct Output {
    func: ImplItemFn,
}

impl Output {
    pub fn from_ast(args: Args, input: Input) -> syn::Result<Self> {
        let inner_fn: ExprClosure = {
            let constness = &input.func.sig.constness;
            let async_block = if input.func.sig.asyncness.is_some() {
                quote! { async move }
            } else {
                quote! { /* non async */ }
            };
            let unsafety = &input.func.sig.unsafety;
            let block = input.func.block;

            parse_quote! {
                #constness move | | { #async_block { #unsafety { #block } } }
            }
        };

        let call_expr: Expr = {
            let output = match &input.func.sig.output {
                ReturnType::Default => {
                    return Err(syn::Error::new(
                        input.func.sig.output.span(),
                        "Result<...> only supported",
                    ))
                }
                ReturnType::Type(_, ty) => ty,
            };
            if input.func.sig.asyncness.is_some() {
                parse_quote! {
                    {
                        let __errify_fn_res: #output = (#inner_fn)().await;
                        __errify_fn_res
                    }
                }
            } else {
                parse_quote! {
                    {
                        let __errify_fn_res: #output = (#inner_fn)();
                        __errify_fn_res
                    }
                }
            }
        };

        let cx_data = match args.cx {
            Context::Explicit(cx) => match cx {
                ExplicitContext::Literal { lit, args } => ContextData::Literal { lit, args },
                ExplicitContext::Expr { expr } => ContextData::Expr { expr },
            },
            Context::Lazy(cx) => match cx {
                LazyContext::Closure { def } => ContextData::Closure { def },
                LazyContext::Function { path } => ContextData::Function { path },
            },
        };
        let cx_expr = context_provider::generic(call_expr, cx_data)?;

        let outer_fn: ImplItemFn = {
            let defaultness = &input.func.defaultness;
            let constness = &input.func.sig.constness;
            let asyncness = &input.func.sig.asyncness;
            let unsafety = &input.func.sig.unsafety;
            let inputs = &input.func.sig.inputs;
            let abi = &input.func.sig.abi;
            let ident = &input.func.sig.ident;
            let (generics_impl, _generics_ty, generics_where) =
                input.func.sig.generics.split_for_impl();
            let ret: ReturnType = {
                let ok = utils::ok_ty(&input.func.sig.output)?;
                let err = error_provider::generic()?;
                parse_quote! { -> ::core::result::Result<#ok, #err> }
            };
            let block: Block = parse_quote! {
                {
                    #cx_expr
                }
            };

            parse_quote! {
                #defaultness #constness #asyncness #unsafety #abi fn #ident #generics_impl ( #inputs ) #ret #generics_where #block
            }
        };

        Ok(Self { func: outer_fn })
    }
}

impl ToTokens for Output {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.func.to_tokens(tokens)
    }
}
