#![allow(unused)]

use std::{
  vec::Vec,
  boxed::Box,
};

macro_rules! func_create {
  ($name:ident,( $($arg:ident: $typ:ty)* )) => {
    pub struct $name<'a> {
      inner: Vec<*mut (dyn Fn($($typ,)* ) + 'a)>
    }
    unsafe impl Sync for $name<'_> {}
    impl<'a> $name<'a> {
      pub const fn new()-> Self {
        $name { inner: Vec::new() }
      }
      pub fn add(&mut self, f: impl Fn($($typ,)*) + 'a)-> usize {
        self.inner.push(Box::into_raw(Box::new(f)));
        self.inner.len() - 1
      }
      pub fn call(&mut self, i: usize, $($arg: $typ,)* ) {
        let f = *self.inner.get_mut(i).unwrap();
        unsafe{ (*f)( $($arg,)* ) }
      }
      pub fn update(&mut self, i: usize, f: impl Fn($($typ,)*) + 'a) {
        let p = self.inner.get_mut(i).unwrap();
        drop(*p);
        *p = Box::into_raw(Box::new(f))
      }
    }
  };
}
func_create!(Funcs0, ());
func_create!(Funcs1, (p1:usize));

#[macro_export]
macro_rules! func_export {
  ($export_name:ident, $class_name:ident, $call_name:ident ($($arg:ident : $typ:ty),*)) => {

    static mut $export_name: crate::funcs::$class_name = crate::funcs::$class_name::new();

    #[no_mangle]
    pub extern fn $call_name(i:usize,$($arg:$typ,)*) {
      unsafe {
        $export_name.call(i,$($arg,)*);
      }
    }

  };
}