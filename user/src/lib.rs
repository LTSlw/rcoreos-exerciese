#![no_std]

#[macro_use]
pub mod console;
pub use console::STDOUT;
mod syscall;
mod lang_items;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    // linkage is not stable
    // but main function in user function required
    extern "C" {
        fn main() -> i32;
    }
    clear_bss();
    unsafe {
        exit(main());
    }
    panic!("unreachable after sys_exit")
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    syscall::sys_write(fd, buf)
}
pub fn exit(exit_code: i32) -> isize {
    syscall::sys_exit(exit_code)
}