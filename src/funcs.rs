
use std::mem::transmute;
macro_rules! param {
  ($t:ident, $f:ident, $c:ident( $($arg:ident : $ty:ty,)* ) ) => {
    type $t = *const dyn Fn( $($ty,)* );
    pub fn $f(f: impl Fn( $($ty,)* ) + 'static)-> [usize;2] {
      unsafe{transmute::<$t, _>(Box::leak(Box::new(f)))}
    }

    #[no_mangle]
    extern fn $c(a:usize,b:usize, $($arg:$ty,)* ) {
      unsafe{(*transmute::<_, $t>([a,b]))( $($arg,)* )}
    }
  };
}
param!(F0, f0, call0());
param!(F1, f1, call1(p1:usize,));
param!(F2, f2, call2(p1:usize,p2:usize,));
