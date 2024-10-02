pub fn console_write_byte(c: u8) {
    sbi_rt::console_write_byte(c);
}

pub fn console_write(bytes: &[u8]) {
    let range = bytes.as_ptr_range();
    let bytes = sbi_rt::Physical::new(bytes.len(), range.start as usize, range.end as usize);
    sbi_rt::console_write(bytes);
}

pub fn shutdown(failure: bool) -> ! {
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    if !failure {
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}