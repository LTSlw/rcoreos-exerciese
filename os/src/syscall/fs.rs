use core::{slice, str};

use crate::batch::check_mem;

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    if !check_mem(buf as usize) || !check_mem(buf as usize + len) {
        return -1;
    }
    match fd {
        FD_STDOUT => {
            let slice = unsafe { slice::from_raw_parts(buf, len) };
            let str = str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            return -1;
        }
    }
}