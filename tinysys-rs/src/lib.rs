#![cfg_attr(not(test), no_std)]

/// Re-export of the sys crate
pub use tinysys_sys as sys;

/// Utilities for sending messages over uart
///
/// This is most useful with `printf`-debugging.
pub mod uart;

#[cfg(target_arch = "riscv32")]
pub fn exit(code: u32) -> ! {
    // TODO: Figure out how to use a return code
    println!("Exiting with code={code}");
    unsafe {
        core::arch::asm! {
            "li a7, 0x5D",
            "ecall",
        };
    }

    unreachable!();
}

#[cfg(not(target_arch = "riscv32"))]
pub fn exit(code: u32) -> ! {
    todo!("Exiting with code={code}");
}

pub mod prelude {
    pub use crate::sys;
    pub use crate::{dbg, print, println};
}

// Re-exports for macros
#[doc(hidden)]
pub mod detail {
    pub use crate::uart::_print;

    pub use embedded_io;
}
