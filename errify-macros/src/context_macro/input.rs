use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Expr, ExprClosure, FnArg, ImplItem, ItemFn, LitStr, Path, Token,
};

pub enum ContextArgs {
    None {
        span: Span,
    },
    Literal {
        lit: LitStr,
        args: Punctuated<Expr, Token![,]>,
    },
    Expr {
        expr: Expr,
    },
}

impl Parse for ContextArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let res = if input.is_empty() {
            Self::None { span: input.span() }
        } else if input.peek(LitStr) {
            let lit = input.parse()?;
            let comma = input.parse::<Option<Token![,]>>()?;
            let args = if comma.is_some() {
                input.parse_terminated(Expr::parse, Token![,])?
            } else {
                Default::default()
            };

            Self::Literal { lit, args }
        } else if let Ok(expr) = input.parse() {
            Self::Expr { expr }
        } else {
            return Err(syn::Error::new(
                input.span(),
                "The macro supports literal with positions arguments and custom error only",
            ));
        };

        if !input.is_empty() {
            return Err(syn::Error::new(input.span(), "Unexpected tokens"));
        }

        Ok(res)
    }
}

pub enum WithContextArgs {
    Closure { def: ExprClosure },
    Function { path: Path },
}

impl Parse for WithContextArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let res = if input.peek(Token![|]) && input.peek2(Token![|]) {
            Self::Closure {
                def: input.parse()?,
            }
        } else if let Ok(path) = input.parse() {
            Self::Function { path }
        } else {
            return Err(syn::Error::new(
                input.span(),
                "The macro supports closure and function only",
            ));
        };

        if !input.is_empty() {
            return Err(syn::Error::new(input.span(), "Unexpected tokens"));
        }

        Ok(res)
    }
}

pub enum Args {
    None {
        span: Span,
    },
    Literal {
        lit: LitStr,
        args: Punctuated<Expr, Token![,]>,
    },
    ErrorType {
        expr: Expr,
    },
    Closure {
        def: ExprClosure,
    },
    Function {
        path: Path,
    },
}

impl From<ContextArgs> for Args {
    fn from(value: ContextArgs) -> Self {
        match value {
            ContextArgs::None { span } => Self::None { span },
            ContextArgs::Literal { lit, args } => Self::Literal { lit, args },
            ContextArgs::Expr { expr } => Self::ErrorType { expr },
        }
    }
}

impl From<WithContextArgs> for Args {
    fn from(value: WithContextArgs) -> Self {
        match value {
            WithContextArgs::Closure { def } => Self::Closure { def },
            WithContextArgs::Function { path } => Self::Function { path },
        }
    }
}

impl Args {
    pub fn span(&self) -> Span {
        match self {
            Args::None { span } => *span,
            Args::Literal { lit, args } => lit
                .span()
                .join(args.span())
                .expect("Arguments taken from one file"),
            Args::ErrorType { expr } => expr.span(),
            Args::Closure { def } => def.span(),
            Args::Function { path } => path.span(),
        }
    }
}

pub enum Input {
    Function(ItemFn),
    Method(ItemFn),
    Impl(ImplItem),
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(f) = input.parse::<ItemFn>() {
            if let Some(FnArg::Receiver(_)) = f.sig.inputs.first() {
                Ok(Self::Method(f))
            } else {
                Ok(Self::Function(f))
            }
        } else if let Ok(i) = input.parse::<ImplItem>() {
            Ok(Self::Impl(i))
        } else {
            Err(syn::Error::new(
                input.span(),
                "The macro supports only impl block or function",
            ))
        }
    }
}