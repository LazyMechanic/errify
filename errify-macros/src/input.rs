use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, ExprClosure, ImplItemFn, LitStr, Path, Token,
};

pub struct ErrifyMacroArgs {
    cx: ImmediateContext,
}

impl Parse for ErrifyMacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self { cx: input.parse()? })
    }
}

pub struct ErrifyWithMacroArgs {
    cx: LazyContext,
}

impl Parse for ErrifyWithMacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self { cx: input.parse()? })
    }
}

pub struct Args {
    pub cx: Context,
}

impl From<ErrifyMacroArgs> for Args {
    fn from(value: ErrifyMacroArgs) -> Self {
        Self {
            cx: value.cx.into(),
        }
    }
}

impl From<ErrifyWithMacroArgs> for Args {
    fn from(value: ErrifyWithMacroArgs) -> Self {
        Self {
            cx: value.cx.into(),
        }
    }
}

pub enum Context {
    Immediate(ImmediateContext),
    Lazy(LazyContext),
}

impl From<ImmediateContext> for Context {
    fn from(value: ImmediateContext) -> Self {
        Self::Immediate(value)
    }
}

impl From<LazyContext> for Context {
    fn from(value: LazyContext) -> Self {
        Self::Lazy(value)
    }
}

pub enum ImmediateContext {
    Literal {
        lit: LitStr,
        args: Punctuated<Expr, Token![,]>,
    },
    Expr {
        expr: Expr,
    },
}

impl Parse for ImmediateContext {
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
