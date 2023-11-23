#[link(wasm_import_module = "base")]
extern {
  fn href()-> usize;
}

mod dom;
use dom::Dom;

mod funcs;
func_export!(FUNCS0, Funcs0, trigger_fn0());
func_export!(FUNCS1, Funcs1, trigger_fn1(p1:usize));

static mut ALLOCATORED_INCOMING: usize = 0;
#[no_mangle]
pub extern fn malloc(size:usize, align:usize)-> usize {
    use std::alloc::{Layout,alloc};
    unsafe {
        let p = alloc(Layout::from_size_align_unchecked(size, align)) as usize;
        ALLOCATORED_INCOMING = p;
        p
    }
}

#[no_mangle]
pub extern fn main() {
    let s = unsafe {
        let len = href();
        let p = ALLOCATORED_INCOMING as *mut u8;
        let s = std::slice::from_raw_parts(p, len);
        std::str::from_utf8_unchecked(s)
    };
    Dom::new().under(Dom::body()).text(s);
}

