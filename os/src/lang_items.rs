use core::panic::PanicInfo;
use crate::{sbi, println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let err = info.message().as_str().unwrap();
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            err
        );
    } else {
        println!("Panicked: {}", err);
    }
    sbi::shutdown(true)
}