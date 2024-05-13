#[doc(hidden)]
#[macro_export]
macro_rules! error {
    ($msg:literal $(,)?) => {
        $crate::__private::format_err(format_args!($msg))
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::__private::Cow::<'static, str>::Owned($crate::__private::format!($fmt, $($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use alloc::borrow::Cow;

    #[test]
    fn literal() {
        let err = error!("literal");
        assert_eq!(err, Cow::Borrowed("literal"));
    }

    #[test]
    fn format_string() {
        let external_named = 1;
        let err = error!("format string {external_named}");
        assert_eq!(
            err,
            Cow::<'static, str>::Owned("format string 1".to_owned())
        );

        let external_named = 1;
        let err = error!(
            "format string {external_named} {internal_named} {}",
            3,
            internal_named = 2
        );
        assert_eq!(
            err,
            Cow::<'static, str>::Owned("format string 1 2 3".to_owned())
        );
    }
}
