#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

/// Re-export of the sys crate
pub use tinysys_sys as sys;

/// Utilities for sending messages over uart
///
/// This is most useful with `printf`-debugging.
pub mod uart;

/// Utilities for debugging
///
/// This includes the kprintln!() macro and friends
pub mod debug;

/// Macro utilities
pub mod macros;

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

#[cfg(feature = "panic-handler")]
#[cfg(target_arch = "riscv32")]
#[panic_handler]
pub fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        // Default palette is VGA, see:
        //      https://retrocomputing.stackexchange.com/questions/27994/
        let kgfx = sys::VPUGetKernelGfxContext();
        sys::VPUSetDefaultPalette(kgfx);
        sys::VPUConsoleSetColors(kgfx, 13 /*text*/, 0 /*bg*/);

        kprintln!(
            "!! {app} has crashed!!  oh no !",
            app = env!("CARGO_PKG_NAME")
        );
        kprintln!("                                                                ");

        if let Some(loc) = info.location() {
            kprintln!("{}:{}:{}: ", loc.file(), loc.line(), loc.column());
        } else {
            // We don't have location info so there's nothing we can put here.
            kprintln!("~somewhere: ");
        }
        kprintln!("                                                                ");

        kprintln!("{}", info.message());

        exit(42);
    }
}

pub mod prelude {
    pub use crate::dbg;
    pub use crate::{kprint, kprintln};
    pub use crate::{print, println};

    pub use crate::sys;
}

// Re-exports for macros
#[doc(hidden)]
pub mod detail {
    pub use crate::macros::_kprint;
    pub use crate::macros::_print;

    pub use embedded_io;
}
