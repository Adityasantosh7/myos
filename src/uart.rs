//! Minimal NS16550A UART driver (the serial port on QEMU's "virt" machine).

use core::fmt;
use core::ptr::{read_volatile, write_volatile};

const UART_BASE: usize = 0x1000_0000;

const REG_DATA: usize = 0; // THR (write) / RBR (read)
const REG_LSR: usize = 5; // line status register

const LSR_RX_READY: u8 = 1 << 0;
const LSR_TX_IDLE: u8 = 1 << 5;

#[derive(Clone, Copy)]
pub struct Uart;

impl Uart {
    #[inline]
    fn reg(offset: usize) -> *mut u8 {
        (UART_BASE + offset) as *mut u8
    }

    /// Send one byte (blocks until the transmitter is ready).
    pub fn putc(&self, byte: u8) {
        unsafe {
            while read_volatile(Self::reg(REG_LSR)) & LSR_TX_IDLE == 0 {}
            write_volatile(Self::reg(REG_DATA), byte);
        }
    }

    /// Read one byte if one is waiting.
    pub fn getc(&self) -> Option<u8> {
        unsafe {
            if read_volatile(Self::reg(REG_LSR)) & LSR_RX_READY != 0 {
                Some(read_volatile(Self::reg(REG_DATA)))
            } else {
                None
            }
        }
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            if b == b'\n' {
                self.putc(b'\r');
            }
            self.putc(b);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!($crate::uart::Uart, $($arg)*);
    }};
}

#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => {{
        $crate::print!($($arg)*);
        $crate::print!("\n");
    }};
}
