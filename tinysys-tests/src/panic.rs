#![no_std]
#![no_main]

extern crate alloc;
extern crate riscv;

// Ensure we link against tinysys_rs
extern crate tinysys_rs;

fn main() {
    // This test makes sure panics behave right. We immediately panic and expect this message to
    // be visible to whoever ran `test-panic`.
    todo!("oh no, where'd the test go? 🙈");
}

/// The entry point loaded by the system
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Optionally initialize a heap.
    // The quickstart example does this because the tinysys system is generally capable of it.
    {
        use core::mem::MaybeUninit;

        #[global_allocator]
        static HEAP: embedded_alloc::LlffHeap = embedded_alloc::LlffHeap::empty();
        // Purposefully tiny heap
        const HEAP_SIZE: usize = 16;

        unsafe {
            #![allow(static_mut_refs)]

            static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
            HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE);
        }
    }

    main();

    tinysys_rs::exit(0);
}
