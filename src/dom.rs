#![allow(unused)]

#[link(wasm_import_module = "base")]
extern {
  fn js_drop(i:usize);
}

macro_rules! ext_fn {
  ($($f:ident)*) => {$(
    fn $f(dom:usize, p:usize, m:usize);
  )*};
}
#[link(wasm_import_module = "dom")]
extern {
  fn get_body()-> usize;
  fn get_window()-> usize;
  fn new_div()-> usize;
  fn append(parent:usize, child:usize);
  fn text(dom:usize, i:usize, l:usize);
  fn style(dom:usize, i:usize, l:usize);
  ext_fn!{
    onclick
    onmousedown2
    onmousemove2
    onmouseup
  }
}


macro_rules! impl_str {
  ($($f:ident)*) => {$(
    pub fn $f(self, s:&str)-> Self {unsafe{
      $f(self.0, s.as_ptr() as usize, s.len())
    }self}
  )*};
}

macro_rules! impl_fn {
  ($($f:ident)*) => {$(
    pub fn $f(self, f:[usize;2])-> Self {unsafe{
      $f(self.0, f[0], f[1]);
    }self}
  )*};
}



#[derive(Clone, Copy)]
pub struct Dom (pub usize);
impl Dom {
  pub fn new()-> Self {Dom(unsafe{new_div()})}
  pub fn body()-> Self {Dom(unsafe{get_body()})}
  pub fn window()-> Self {Dom(unsafe{get_window()})}
  // Instances
  pub fn drop(self) {unsafe{
    js_drop(self.0);
  }}
  pub fn append(self, child:Self)-> Self {unsafe{
    append(self.0, child.0)
  }self}
  pub fn under(self, parent:Self)-> Self {unsafe{
    append(parent.0, self.0)
  }self}
  impl_str!{
    text
    style
  }
  impl_fn!{
    onclick
    onmousedown2
    onmousemove2
    onmouseup
  }
}
