
#![allow(unused)]

#[link(wasm_import_module = "base")]
extern {
  fn href()-> usize;
  fn log(a:usize);
}

mod dom;
use dom::Dom;

mod funcs;
use funcs::{f0,f1,f2};

mod js_str;

#[inline]
fn leak<T>(e:T)-> *mut T {
  Box::leak(Box::new(e)) as *mut T
}

#[no_mangle]
extern fn main() {unsafe{
  let fn0_null = f0(||{});
  let fn2_null = f2(|_p1:usize,_p2:usize|{});

  let win = Dom::window();

  let d = Dom::new();
  let prev_pos = leak([0isize;2]);
  let cur_pos = leak([0isize;2]);
  let mousemove = f2(move|x,y|{
    let (x,y) = (x as isize, y as isize);
    let [cx, cy] = *cur_pos;
    let [px, py] = *prev_pos;
    *cur_pos = [cx+x-px, cy+y-py];
    *prev_pos = [x, y];
    let [cx, cy] = *cur_pos;
    d.style(format!("left:{}px;top:{}px;", cx, cy).as_str());
  });
  let mouseup = f0(move||{
    win.onmousemove2(fn2_null);
    win.onmouseup(fn0_null);
  });
  d.under(Dom::body()).text(fetch_str!(href()))
  .onmousedown2(f2(move|x,y|{
    *prev_pos = [x as isize, y as isize];
    win.onmousemove2(mousemove);
    win.onmouseup(mouseup);
  }));

}}

