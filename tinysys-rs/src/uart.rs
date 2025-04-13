#![allow(unused_imports)]

use crate::sys;

use embedded_io::{ErrorType, Write};

pub struct UartWriter;

impl UartWriter {
    pub fn finish_line(&mut self) {
        let _ = self.write_all(b"\n");
    }
}

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
