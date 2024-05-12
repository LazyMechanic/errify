use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, ExprClosure, ImplItemFn, LitStr, Path, Token,
};

pub struct ErrifyMacroArgs {
    err_ty: Option<Path>,
    cx: ExplicitContext,
}

impl Parse for ErrifyMacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // TODO:
        // let err_ty = input.parse()?;
        // if err_ty.is_some() {
        //     let _ = input.parse::<Token![,]>()?;
        // }
        let err_ty = None;
        let cx = input.parse()?;

        Ok(Self { err_ty, cx })
    }
}

pub struct ErrifyWithMacroArgs {
    err_ty: Option<Path>,
    cx: LazyContext,
}

impl Parse for ErrifyWithMacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // TODO:
        // let err_ty = input.parse()?;
        // if err_ty.is_some() {
        //     let _ = input.parse::<Token![,]>()?;
        // }
        let err_ty = None;
        let cx = input.parse()?;

        Ok(Self { err_ty, cx })
    }
}

pub struct Args {
    pub err_ty: Option<Path>,
    pub cx: Context,
}

impl From<ErrifyMacroArgs> for Args {
    fn from(value: ErrifyMacroArgs) -> Self {
        Self {
            err_ty: value.err_ty,
            cx: value.cx.into(),
        }
    }
}

impl From<ErrifyWithMacroArgs> for Args {
    fn from(value: ErrifyWithMacroArgs) -> Self {
        Self {
            err_ty: value.err_ty,
            cx: value.cx.into(),
        }
    }
}

pub enum Context {
    Explicit(ExplicitContext),
    Lazy(LazyContext),
}

impl From<ExplicitContext> for Context {
    fn from(value: ExplicitContext) -> Self {
        Self::Explicit(value)
    }
}

impl From<LazyContext> for Context {
    fn from(value: LazyContext) -> Self {
        Self::Lazy(value)
    }
}

pub enum ExplicitContext {
    Literal {
        lit: LitStr,
        args: Punctuated<Expr, Token![,]>,
    },
    Expr {
        expr: Expr,
    },
}

impl Parse for ExplicitContext {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let res = if input.peek(LitStr) {
            let lit = input.parse()?;
            let comma = input.parse::<Option<Token![,]>>()?;
            let args = if comma.is_some() {
                input.parse_terminated(Expr::parse, Token![,])?
            } else {
                Default::default()
            };

            Self::Literal { lit, args }
        } else {
            Self::Expr {
                expr: input.parse()?,
            }
        };

        if !input.is_empty() {
            return Err(syn::Error::new(input.span(), "Unexpected tokens"));
        }

        Ok(res)
    }
}

pub enum LazyContext {
    Closure { def: ExprClosure },
    Function { path: Path },
}

impl Parse for LazyContext {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let res = if input.peek(Token![|]) && input.peek2(Token![|]) {
            Self::Closure {
                def: input.parse()?,
            }
        } else {
            Self::Function {
                path: input.parse()?,
            }
        };

        if !input.is_empty() {
            return Err(syn::Error::new(input.span(), "Unexpected tokens"));
        }

        Ok(res)
    }
}

pub struct Input {
    pub func: ImplItemFn,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            func: input.parse()?,
        })
    }
}
