use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, ToTokens};
use syn::{
    punctuated::Punctuated, spanned::Spanned, Attribute, FnArg, GenericArgument, Pat,
    PathArguments, ReturnType, Token, Type,
};

pub fn ok_ty(return_ty: &ReturnType) -> syn::Result<Type> {
    let err = |span: Span| syn::Error::new(span, "Invalid return type. Expected `Result<...>`");

    let ReturnType::Type(_arrow, ty) = return_ty else {
        return Err(err(return_ty.span()));
    };

    let Type::Path(ref ty) = **ty else {
        return Err(err(ty.span()));
    };

    let pathless_ty = ty.path.segments.last().ok_or_else(|| err(ty.span()))?;

    let PathArguments::AngleBracketed(args) = &pathless_ty.arguments else {
        return Err(err(pathless_ty.span()));
    };

    let generic_arg = args
        .args
        .first()
        .ok_or_else(|| syn::Error::new(args.span(), "`Ok` type of `Result<Ok, Err>` not found"))?;

    let GenericArgument::Type(ok_ty) = generic_arg else {
        return Err(err(generic_arg.span()));
    };

    Ok(ok_ty.clone())
}

pub fn clear_inputs<'a>(
    inputs: impl IntoIterator<Item = &'a FnArg>,
) -> syn::Result<Punctuated<CleanFnArg, Token![,]>> {
    inputs
        .into_iter()
        .enumerate()
        .map(|(idx, input)| match input {
            FnArg::Receiver(_) => Err(syn::Error::new(
                input.span(),
                "`self` argument not supported",
            )),
            FnArg::Typed(pat_ty) => {
                let ident = match &*pat_ty.pat {
                    Pat::Ident(ident) => ident.ident.clone(),
                    _ => format_ident!("arg{idx}"),
                };

                Ok(CleanFnArg {
                    attrs: pat_ty.attrs.clone(),
                    ident,
                    colon_token: Default::default(),
                    ty: pat_ty.ty.clone(),
                })
            }
        })
        .collect()
}

pub fn call_inputs<'a>(
    inputs: impl IntoIterator<Item = &'a CleanFnArg>,
) -> Punctuated<Ident, Token![,]> {
    inputs
        .into_iter()
        .map(|input| input.ident.clone())
        .collect()
}

pub struct CleanFnArg {
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub colon_token: Token![:],
    pub ty: Box<Type>,
}

impl ToTokens for CleanFnArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for attr in &self.attrs {
            attr.to_tokens(tokens)
        }

        self.ident.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}
