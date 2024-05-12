use proc_macro2::Span;
use syn::{parse_quote, punctuated::Punctuated, Expr, ExprClosure, LitStr, Path, Token};

pub enum ContextData {
    Literal {
        lit: LitStr,
        args: Punctuated<Expr, Token![,]>,
    },
    Expr {
        expr: Expr,
    },
    Closure {
        def: ExprClosure,
    },
    Function {
        path: Path,
    },
}

#[allow(clippy::needless_return)]
#[allow(unreachable_code)]
pub fn generic(call_expr: Expr, data: ContextData) -> syn::Result<Expr> {
    if cfg!(feature = "anyhow") && cfg!(feature = "eyre") {
        return Err(syn::Error::new(
            Span::call_site(),
            "Ambiguous using errify_macro provider. Choose either `anyhow` or `eyre`",
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
        return Ok(anyhow(call_expr, data));
    }

    #[cfg(feature = "eyre")]
    {
        return Ok(eyre(call_expr, data));
    }
}

#[cfg(feature = "anyhow")]
pub fn anyhow(call_expr: Expr, data: ContextData) -> Expr {
    match data {
        ContextData::Literal { lit, args } => parse_quote! {
            {
                let __errify_cx = ::errify::__private::anyhow::anyhow!(#lit, #args);
                ::errify::__private::anyhow::Context::context( #call_expr, __errify_cx )
            }
        },
        ContextData::Expr { expr } => parse_quote! {
            {
                let __errify_cx = #expr;
                ::errify::__private::anyhow::Context::context( #call_expr, __errify_cx )
            }
        },
        ContextData::Closure { def } => parse_quote! {
            {
                let __errify_cx = #def;
                ::errify::__private::anyhow::Context::with_context( #call_expr, __errify_cx )
            }
        },
        ContextData::Function { path } => parse_quote! {
            ::errify::__private::anyhow::Context::with_context( #call_expr, #path )
        },
    }
}

#[cfg(feature = "eyre")]
pub fn eyre(call_expr: Expr, data: ContextData) -> Expr {
    match data {
        ContextData::Literal { lit, args } => parse_quote! {
            {
                let __errify_cx = ::errify::__private::eyre::eyre!(#lit, #args);
                ::errify::__private::eyre::WrapErr::wrap_err( #call_expr, __errify_cx )
            }
        },
        ContextData::Expr { expr } => parse_quote! {
            {
                let __errify_cx = #expr;
                ::errify::__private::eyre::WrapErr::wrap_err( #call_expr, __errify_cx )
            }
        },
        ContextData::Closure { def } => parse_quote! {
            {
                let __errify_cx = #def;
                ::errify::__private::eyre::WrapErr::wrap_err_with( #call_expr, __errify_cx )
            }
        },
        ContextData::Function { path } => parse_quote! {
            ::errify::__private::eyre::WrapErr::wrap_err_with( #call_expr, #path )
        },
    }
}
