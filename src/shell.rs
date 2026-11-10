//! A tiny interactive shell over the serial port.

use crate::uart::Uart;
use crate::{print, println};
use core::ptr::{read_volatile, write_volatile};

const PROMPT: &str = "myos> ";
const MAX_LINE: usize = 128;

// QEMU "virt" machine devices
const CLINT_MTIME: usize = 0x0200_BFF8; // free-running timer
const MTIME_HZ: u64 = 10_000_000; // 10 MHz on virt
const TEST_FINISHER: usize = 0x10_0000; // writing 0x5555 powers off the VM

pub fn run() -> ! {
    let uart = Uart;
    let mut buf = [0u8; MAX_LINE];
    let mut len = 0usize;

    print!("{}", PROMPT);
    loop {
        let Some(c) = uart.getc() else { continue };
        match c {
            b'\r' | b'\n' => {
                println!();
                if let Ok(line) = core::str::from_utf8(&buf[..len]) {
                    execute(line.trim());
                }
                len = 0;
                print!("{}", PROMPT);
            }
            // backspace / delete
            0x08 | 0x7f => {
                if len > 0 {
                    len -= 1;
                    print!("\x08 \x08");
                }
            }
            // printable ASCII
            0x20..=0x7e => {
                if len < MAX_LINE {
                    buf[len] = c;
                    len += 1;
                    uart.putc(c);
                }
            }
            _ => {}
        }
    }
}

fn execute(line: &str) {
    if line.is_empty() {
        return;
    }
    let (cmd, rest) = match line.split_once(' ') {
        Some((c, r)) => (c, r.trim()),
        None => (line, ""),
    };

    match cmd {
        "help" => {
            println!("Commands:");
            println!("  help       show this list");
            println!("  clear      clear the screen");
            println!("  echo TEXT  print TEXT");
            println!("  info       about this OS");
            println!("  uptime     time since boot");
            println!("  panic      trigger a kernel panic (test)");
            println!("  shutdown   power off the virtual machine");
        }
        "clear" => print!("\x1b[2J\x1b[H"),
        "echo" => println!("{}", rest),
        "info" => {
            println!("MyOS v0.1");
            println!("  arch    : RISC-V 64 (rv64gc)");
            println!("  machine : QEMU virt");
            println!("  language: Rust (no_std)");
        }
        "uptime" => {
            let ticks = unsafe { read_volatile(CLINT_MTIME as *const u64) };
            let ms = ticks * 1000 / MTIME_HZ;
            println!("up {}.{:03} s", ms / 1000, ms % 1000);
        }
        "panic" => panic!("user requested panic"),
        "shutdown" => {
            println!("Powering off...");
            unsafe { write_volatile(TEST_FINISHER as *mut u32, 0x5555) };
            loop {
                unsafe { core::arch::asm!("wfi") };
            }
        }
        _ => println!("unknown command: {} (try 'help')", cmd),
    }
}
