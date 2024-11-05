mod context;
pub use context::TrapContext;
use log::error;

use core::arch::global_asm;

use riscv::{interrupt::{Exception, Trap}, register::{scause, stval, stvec::{self, TrapMode}}};

use crate::{batch::run_next_app, syscall};

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(e) if e == Exception::UserEnvCall as usize => {
            cx.sepc += 4;
            cx.x[10] = syscall::syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(e) if e == Exception::StoreFault as usize || e == Exception::StorePageFault as usize => {
            error!(target: "kernel", "PageFault in application, kernel killed it.");
            run_next_app();
        }
        Trap::Exception(e) if e == Exception::IllegalInstruction as usize => {
            error!(target: "kernel", "IllegalInstruction in application, kernel killed it.");
            run_next_app();
        }
        _ => {
            panic!("un supported strp {:?}, stval = {:#x}!", scause.cause(), stval);
        }
    }
    cx
}