#![cfg_attr(not(test), no_std)]

/// Re-export of the sys crate
pub use tinysys_sys as sys;

/// Utilities for sending messages over uart
///
/// This is most useful with `printf`-debugging.
pub mod uart;

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
