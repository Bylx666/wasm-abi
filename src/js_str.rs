#![allow(unused_unsafe)]

pub static mut ALLOCATORED_INCOMING: usize = 0;
#[no_mangle]
extern fn malloc(size:usize, align:usize)-> usize {
    use std::alloc::{Layout,alloc};
    unsafe {
        let p = alloc(Layout::from_size_align_unchecked(size, align)) as usize;
        ALLOCATORED_INCOMING = p;
        p
    }
}

#[macro_export]
macro_rules! fetch_str {
  ($len:ident) => {{
    let p = crate::js_str::ALLOCATORED_INCOMING as *mut u8;
    let s = std::slice::from_raw_parts(p, $len);
    std::str::from_utf8_unchecked(s)
  }};
  ($len:ident()) => {{
    let len = $len();
    fetch_str!(len)
  }};
}