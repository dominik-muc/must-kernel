use core::fmt::Write;

const UART: *mut u8 = 0x10000000 as *mut u8;
const UART_LSR: *mut u8 = 0x10000005 as *mut u8;

pub struct UartWriter;

impl UartWriter {
    pub fn putchar(c: u8) {
        unsafe {
            core::ptr::write_volatile(UART, c as u8);
        }
    }

    pub fn getchar() -> u8 {
        unimplemented!()
    }
}

impl Write for UartWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.as_bytes() {
            Self::putchar(*c);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! uprintln {
    ($($arg:tt)*) => {
        writeln!(UartWriter, $($arg)*).unwrap_or(())
    }
}

#[macro_export]
macro_rules! uprint {
    ($($arg:tt)*) => {
        write!(UartWriter, $($arg)*).unwrap_or(())
    }
}
