#![allow(unused_imports)]

use crate::sys;

use embedded_io::{ErrorType, Write};

pub struct UartWriter;

impl ErrorType for UartWriter {
    // UARTSendBlock() doesn't report an error, so we'll never raise an error.
    type Error = core::convert::Infallible;
}

#[cfg(target_arch = "riscv32")]
impl Write for UartWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        unsafe {
            sys::UARTSendBlock(buf.as_ptr() as *mut u8, buf.len() as u32);
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

// Off of riscv32, we have no bindings.
// To keep things working smoothly, assume we have std and write to stdout.
#[cfg(not(target_arch = "riscv32"))]
impl Write for UartWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        use std::io::Write;

        std::io::stdout().write_all(buf).unwrap();
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

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
