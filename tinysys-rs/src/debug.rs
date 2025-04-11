#[allow(unused_imports)]
use super::sys::*;

use embedded_io::{ErrorType, Write};

/// This writes text to the screen without much consideration for what else is going on.
#[derive(Debug)]
pub struct KernelDebugWriter;

impl ErrorType for KernelDebugWriter {
    // VPUConsolePrint() doesn't report an error, so we'll never raise an error.
    type Error = core::convert::Infallible;
}

#[cfg(target_arch = "riscv32")]
impl Write for KernelDebugWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        unsafe {
            VPUConsolePrint(VPUGetKernelGfxContext(), buf.as_ptr(), buf.len() as i32);
            Ok(buf.len())
        }
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

// Off of riscv32, we have no bindings.
// To keep things working smoothly, assume we have std and write to stdout.
#[cfg(not(target_arch = "riscv32"))]
impl Write for KernelDebugWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        use std::io::Write;

        std::io::stdout().write_all(buf).unwrap();
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
