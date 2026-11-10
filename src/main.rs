#![no_std]
#![no_main]

mod shell;
mod uart;

use core::arch::global_asm;
use core::panic::PanicInfo;

// Boot code: sets up the stack, clears .bss, then calls `kmain`.
global_asm!(include_str!("boot.S"));

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    println!();
    println!("MyOS v0.1 - RISC-V 64 / Rust");
    println!("Booted on hart 0. Type 'help' for commands.");
    println!();
    shell::run()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!();
    println!("*** KERNEL PANIC ***");
    println!("{}", info);
    loop {
        unsafe { core::arch::asm!("wfi") };
    }
}
