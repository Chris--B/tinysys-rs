#![no_std]
#![no_main]

extern crate alloc;
// link against panic handler
extern crate panic_halt;
// link against critical section impl
extern crate riscv;

use tinysys_rs::prelude::*;

fn main() {
    println!("### Hello println!() tests 👋");
    print!("(1) Hello ");
    print!("(2) World ");
    print!("\n");

    let x = 1;
    let y = 2;
    dbg!();
    dbg!(x);
    println!();

    let (xx, yy) = dbg!((x, y));
    dbg!(xx);
    dbg!(yy);
    println!();

    // Check something that's not Copy
    let v = alloc::vec![1, 2, 3_i16];
    let v = dbg!(v);
    println!();

    kprintln!("### kprintln!() tests");
    kprint!("(3) Hello ");
    kprint!("(4) World ");
    kprint!("\n");
    kprintln!();
    kprintln!("v = {v:#?}");
    kprintln!();
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
