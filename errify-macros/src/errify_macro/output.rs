use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_quote, spanned::Spanned, Block, Expr, ExprClosure, ImplItemFn, ReturnType, Type};

use crate::{
    errify_macro::input::{Args, Context, ExplicitContext, Input, LazyContext},
    utils,
};

pub struct Output {
    func: ImplItemFn,
}

impl Output {
    pub fn from_ast(args: Args, input: Input) -> syn::Result<Self> {
        let inner_fn: ExprClosure = {
            let constness = &input.func.sig.constness;
            let unsafety = &input.func.sig.unsafety;
            let async_block = if input.func.sig.asyncness.is_some() {
                quote! { async move }
            } else {
                quote! { /* non async */ }
            };
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
                        let __errify_fn = #inner_fn;
                        let __errify_fn_res: #output = (__errify_fn)().await;
                        __errify_fn_res
                    }
                }
            } else {
                parse_quote! {
                    {
                        let __errify_fn = #inner_fn;
                        let __errify_fn_res: #output = (__errify_fn)();
                        __errify_fn_res
                    }
                }
            }
        };

        let err_ty = match args.err_ty {
            #[allow(unreachable_code)]
            None => 'err_ty: {
                if cfg!(feature = "anyhow") && cfg!(feature = "eyre") {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        "Ambiguous error type. Choose either `anyhow` or `eyre`",
                    ));
                }

                if !cfg!(feature = "anyhow") && !cfg!(feature = "eyre") {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        "None of the `anyhow` or `eyre` features are enabled",
                    ));
                }

                #[cfg(feature = "anyhow")]
                {
                    break 'err_ty parse_quote! { ::errify::__private::anyhow::Error };
                }

                #[cfg(feature = "eyre")]
                {
                    break 'err_ty parse_quote! { ::errify::__private::eyre::Report };
                }
            }
            Some(ty) => ty,
        };

        let cx_expr = apply_context(&call_expr, &args.cx, &err_ty);

        let outer_fn: ImplItemFn = {
            let attrs = &input.func.attrs;
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
                let err = err_ty;
                parse_quote! { -> ::errify::__private::Result<#ok, #err> }
            };
            let block: Block = parse_quote! {
                {
                    #cx_expr
                }
            };

            parse_quote! {
                #(#attrs)*
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

pub fn apply_context(call_expr: &Expr, cx: &Context, err_ty: &Type) -> Expr {
    match cx {
        Context::Explicit(ExplicitContext::Literal { lit, args }) => parse_quote! {
            {
                let __errify_cx = ::errify::error!(#lit, #args);
                let __errify_res = #call_expr;
                match __errify_res {
                    ::errify::__private::Ok(v) => Ok(v),
                    ::errify::__private::Err(err) => Err(<#err_ty as ::errify::WrapErr<_>>::wrap_err(err, __errify_cx)),
                }
            }
        },
        Context::Explicit(ExplicitContext::Expr { expr }) => parse_quote! {
            {
                let __errify_cx = #expr;
                let __errify_res = #call_expr;
                match __errify_res {
                    ::errify::__private::Ok(v) => Ok(v),
                    ::errify::__private::Err(err) => Err(<#err_ty as ::errify::WrapErr<_>>::wrap_err(err, __errify_cx)),
                }
            }
        },
        Context::Lazy(LazyContext::Closure { def }) => parse_quote! {
            {
                let __errify_cx = #def;
                let __errify_res = #call_expr;
                match __errify_res {
                    ::errify::__private::Ok(v) => Ok(v),
                    ::errify::__private::Err(err) => Err(<#err_ty as ::errify::WrapErr<_>>::wrap_err(err, (__errify_cx)())),
                }
            }
        },
        Context::Lazy(LazyContext::Function { path }) => parse_quote! {
            {
                let __errify_res = #call_expr;
                match __errify_res {
                    ::errify::__private::Ok(v) => Ok(v),
                    ::errify::__private::Err(err) => Err(<#err_ty as ::errify::WrapErr<_>>::wrap_err(err, #path())),
                }
            }
        },
    }
}
