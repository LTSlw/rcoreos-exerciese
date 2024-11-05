#![no_std]
#![no_main]

mod lang_items;
mod sbi;
mod logger;

#[macro_use]
mod console;

mod batch;
mod sync;
mod trap;
mod syscall;

use core::arch::global_asm;
use log::{debug, error, info, trace, warn};

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logger::init();

    print_system_info();
    print_to_console();

    trap::init();
    batch::init();
    batch::run_next_app();
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

fn print_to_console() {
    sbi::console_write_byte('O' as u8);
    sbi::console_write_byte('K' as u8);
    sbi::console_write_byte('\n' as u8);

    trace!("Hello World.");
    debug!("Hello World.");
    info!("Hello World.");
    warn!("Hello World.");
    error!("Hello World.");
}

fn print_system_info() {
    extern "C" {
        fn stext();
        fn etext();
        fn sdata();
        fn edata();
        fn srodata();
        fn erodata();
        fn sbss();
        fn ebss();
    }

    info!(".text [{:#x}, {:#x}]", stext as usize, etext as usize);
    info!(".rodata [{:#x}, {:#x}]", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x}]", sdata as usize, edata as usize);
    info!(".bss [{:#x}, {:#x}]", sbss as usize, ebss as usize);
}