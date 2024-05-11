pub use errify_derive::{context, with_context};

#[doc(hidden)]
pub mod __private {
    #[cfg(feature = "anyhow")]
    pub use anyhow;
    #[cfg(feature = "eyre")]
    pub use eyre;
}
