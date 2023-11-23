#![allow(unused)]

#[link(wasm_import_module = "base")]
extern {
  fn js_drop(i:usize);
}

#[link(wasm_import_module = "dom")]
extern {
  fn get_body()-> usize;
  fn new_div()-> usize;
  fn append(parent:usize, child:usize);
  fn text(dom:usize, i:usize, l:usize);
}


pub struct Dom (usize);
impl Dom {
  pub fn body()-> Self {unsafe{Dom(get_body())}}
  pub fn new()-> Self {unsafe{Dom(new_div())}}
  // Instances
  pub fn append(self, child:Self)-> Self {
    unsafe{append(self.0, child.0)}self
  }
  pub fn under(self, parent:Self)-> Self {
    unsafe{append(parent.0, self.0)}self
  }
  pub fn text(self, s:&str)-> Self {
    unsafe{text(self.0, s.as_ptr() as usize, s.len())}self
  }
}
impl Drop for Dom {
  fn drop(&mut self) {unsafe{js_drop(self.0)}}
}
