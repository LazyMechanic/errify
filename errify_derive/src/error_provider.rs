use proc_macro2::Span;
use syn::{parse_quote, Type};

#[allow(clippy::needless_return)]
#[allow(unreachable_code)]
pub fn generic() -> syn::Result<Type> {
    if cfg!(feature = "anyhow") && cfg!(feature = "eyre") {
        return Err(syn::Error::new(
            Span::call_site(),
            "Ambiguous using error provider. Choose either `anyhow` or `eyre`",
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
        return Ok(anyhow());
    }

    #[cfg(feature = "eyre")]
    {
        return Ok(eyre());
    }
}

#[cfg(feature = "anyhow")]
pub fn anyhow() -> Type {
    parse_quote! { ::errify::__private::anyhow::Error }
}

#[cfg(feature = "eyre")]
pub fn eyre() -> Type {
    parse_quote! { ::errify::__private::eyre::Report }
}
