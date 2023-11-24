#[link(wasm_import_module = "base")]
extern {
  fn href()-> usize;
  fn log(a:usize);
}

mod dom;
use dom::Dom;

mod funcs;
func_export!(FUNCS0, Funcs0, trigger_fn0());
func_export!(FUNCS1, Funcs1, trigger_fn1(p1:usize));
func_export!(FUNCS2, Funcs2, trigger_fn2(p1:usize, p2:usize));

mod js_str;

#[inline]
fn leak<T>(e:T)-> *mut T {
  Box::leak(Box::new(e)) as *mut T
}

#[no_mangle]
extern fn main() {unsafe{
  let fn0_null = FUNCS0.add(||{});
  let fn2_null = FUNCS2.add(|_p1:usize,_p2:usize|{});

  let win = Dom::window();

  #[derive(Clone, Copy)]
  struct Pos (*mut [isize;2]);
  impl Pos {
    fn new()-> Self {
      Pos(leak([0isize;2]))
    }
    fn get(&self)-> [isize;2] {
      unsafe{*self.0}
    }
    fn set(&self, x:isize, y:isize) {
      unsafe{(*self.0)=[x,y]}
    }
  }

  let d = Dom::new();
  let prev_pos = Pos::new();
  let cur_pos = Pos::new();
  let mousemove = FUNCS2.add(move|x,y|{
    let (x,y) = (x as isize, y as isize);
    let [cx, cy] = cur_pos.get();
    let [px, py] = prev_pos.get();
    cur_pos.set(cx+x-px, cy+y-py);
    prev_pos.set(x, y);
    let [cx, cy] = cur_pos.get();
    d.style(format!("left:{}px;top:{}px;", cx, cy).as_str());
  });
  let mouseup = FUNCS0.add(move||{
    win.onmousemove2(fn2_null);
    win.onmouseup(fn0_null);
  });
  d.under(Dom::body()).text(fetch_str!(href()))
  .onmousedown2(FUNCS2.add(move|x,y|{
    prev_pos.set(x as isize, y as isize);
    log(x);
    win.onmousemove2(mousemove);
    win.onmouseup(mouseup);
  }));

}}

