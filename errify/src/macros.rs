#[doc(hidden)]
#[macro_export]
macro_rules! error {
    ($err:ty, $msg:literal $(,)?) => {
        $crate::__private::format_err::<$err>(::core::format_args!($msg))
    };
    ($err:ty, $fmt:expr, $($arg:tt)*) => {
        <$err as $crate::FromMessage>::from_msg($crate::__private::format!($fmt, $($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use core::fmt::{Debug, Display};

    use crate::FromMessage;

    #[derive(Debug, Eq, PartialEq)]
    struct CustomError(String);

    impl FromMessage for CustomError {
        fn from_msg<M>(msg: M) -> Self
        where
            M: Display + Debug + Send + Sync + 'static,
        {
            Self(msg.to_string())
        }
    }

    #[test]
    fn literal() {
        let err = error!(CustomError, "literal");
        assert_eq!(err, CustomError("literal".into()))
    }

    #[test]
    fn format_string() {
        let external_named = 1;
        let err = error!(
            CustomError,
            "format string {external_named} {internal_named} {}",
            3,
            internal_named = 2
        );
        assert_eq!(err, CustomError("format string 1 2 3".into()))
    }
}
