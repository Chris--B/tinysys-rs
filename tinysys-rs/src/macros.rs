use crate::debug::KernelDebugWriter;
use crate::uart::UartWriter;

#[macro_export]
macro_rules! dbg {
    () => {
        {
            use $crate::detail::embedded_io::Write;

            let loc: &str = concat!("[", file!(), ":", line!(), ":", column!(), "]");
            println!("{loc}");
        }
    };

    ($val:expr $(,)?) => {
        {
            use $crate::detail::embedded_io::Write;

            let loc: &str = concat!("[", file!(), ":", line!(), ":", column!(), "]");
            let val = $val;
            println!("{loc} {} = {val:#?}", stringify!($val));
            val
        }
    };

    ($($val:expr),+ $(,)?) => {
        {
            use $crate::detail::embedded_io::Write;

            let loc: &str = concat!("[", file!(), ":", line!(), ":", column!(), "]");
            print!("{loc} ");
            $(
                {
                    let val = $val;
                    print!("{} = {val:#?}, ", stringify!($val));
                },
            )+
            println!();

            // "Return" the expression list as a tuple
            // Note: This evaluates $val twice! Hopefully it wasn't doing anything weird.
            (
                $(
                    $val,
                )+
            )
        }
    };
}

pub fn _print(args: core::fmt::Arguments) {
    use embedded_io::Write;

    let _ = write!(UartWriter, "{}", args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::detail::_print(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };

    ($($arg:tt)*) => {{
        $crate::print!($($arg)*);
        $crate::print!("\n")
    }};
}

pub fn _kprint(args: core::fmt::Arguments) {
    use embedded_io::Write;

    let _ = write!(KernelDebugWriter, "{}", args);
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {{
        $crate::detail::_kprint(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! kprintln {
    () => {
        $crate::kprint!("\n")
    };

    ($($arg:tt)*) => {{
        $crate::kprint!($($arg)*);
        $crate::kprint!("\n")
    }};
}
