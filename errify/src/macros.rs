#[macro_export]
macro_rules! error {
    ($err:ty, $msg:literal $(,)?) => {
        $crate::__private::format_err::<$err>(::core::format_args!($msg))
    };
    ($err:ty, $fmt:expr, $($arg:tt)*) => {
        <$err as $crate::Error>::msg($crate::__private::format!($fmt, $($arg)*))
    };
}
