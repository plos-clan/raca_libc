#![no_std]

use core::alloc::Layout;

use alloc::alloc::{alloc, dealloc};

extern crate alloc;
extern crate core;
extern crate raca_std;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn read(fd: i32, buf: *mut u8, count: usize) -> isize {
    unsafe {
        raca_std::fs::File::from_raw_fd(fd as usize)
            .read(core::slice::from_raw_parts_mut(buf, count))
            .map(|n| n as isize)
            .unwrap_or(-1)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn write(fd: i32, buf: *const u8, count: usize) -> isize {
    unsafe {
        raca_std::fs::File::from_raw_fd(fd as usize)
            .write(core::slice::from_raw_parts(buf, count))
            .map(|n| n as isize)
            .unwrap_or(-1)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
    unsafe { alloc(Layout::from_size_align(size, 1).unwrap()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free(ptr: *mut u8) {
    unsafe { dealloc(ptr, Layout::from_size_align(0, 1).unwrap()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn exit(code: i32) -> ! {
    raca_std::process::exit(code as usize)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(ptr: *mut u8, value: u8, count: usize) {
    unsafe {
        core::ptr::write_bytes(ptr, value, count);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, count: usize) {
    unsafe {
        core::ptr::copy_nonoverlapping(src, dest, count);
    }
}
