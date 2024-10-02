use core::panic::PanicInfo;
use crate::{sbi, println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().as_str().unwrap()
        );
    } else {
        println!("Panicked: {}", info.message().as_str().unwrap());
    }
    sbi::shutdown(true)
}