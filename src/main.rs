#![no_std]
#![no_main]

mod lang_items;
mod sbi;
mod logger;

#[macro_use]
mod console;

use core::arch::global_asm;
use log::{debug, error, info, trace, warn};

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logger::init();

    sbi::console_write_byte('O' as u8);
    sbi::console_write_byte('K' as u8);
    sbi::console_write_byte('\n' as u8);

    trace!("Hello World.");
    debug!("Hello World.");
    info!("Hello World.");
    warn!("Hello World.");
    error!("Hello World.");

    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}