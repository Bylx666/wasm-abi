
// (()=>{

// frames
const {log} = console;
const utf8 = {
  dec:TextDecoder.prototype.decode.bind(new TextDecoder),
  enc:TextEncoder.prototype.encode.bind(new TextEncoder)
};


// module
let mod = {
  // built
  mem: new ArrayBuffer(0),
  alloc(size,align) {return 0;},
  call0(p,m) {},
  call1(p,m,p1) {},
  call2(p,m,p1,p2) {},

  // static
  get_str(i, j) {
    return utf8.dec(new Uint8Array(this.mem,i,j));
  },
  put(s) {
    let l = s.byteLength;
    let p = this.alloc(l, 8);
    new Uint8Array(this.mem).set(u8, p);
    return l;
  }

};


// pointers
let ptr = 0;
let heap = {
  add: (e)=> {
    if(typeof heap[ptr]==="number") {
      let p = heap[ptr];
      heap[ptr] = e;
      ptr = p;
    }else {
      heap[++ptr] = e;
    }
    return ptr;
  },
  drop: (i)=> {
    heap[i] = ptr;
    ptr = i;
  }
};


// apis
const base = {
  js_drop: (i)=> heap.drop(i),
  js_str: (p, l)=> heap.add(mod.get_str(p, l)),
  log: (i)=> log(i),
  href: ()=> heap.add(location.href)
};

const HEAP_WIN = heap.add(window);
const HEAP_BODY = heap.add(document.body);
const dom = {
  get_window: ()=> HEAP_WIN,
  get_body: ()=> HEAP_BODY,
  new_div: ()=> heap.add(document.createElement("div")),
  append: (i, j)=> heap[i].append(heap[j]),
  text: (i, j)=> heap[i].textContent = heap[j],
  onclick: (i, p, m)=> heap[i].onclick = ()=> mod.call0(p, m),
  onmousedown2: (i, p, m)=> heap[i].onmousedown = (e)=> mod.call2(p, m, e.clientX, e.clientY),
  onmousemove2: (i, p, m)=> heap[i].onmousemove = (e)=> mod.call2(p, m, e.clientX, e.clientY),
  onmouseup: (i, p, m)=> heap[i].onmouseup = ()=> mod.call0(p, m),
  style: (i, j)=> heap[i].style.cssText = heap[j]
};


// initing
let imports = {
  base, dom
};

WebAssembly.instantiateStreaming(fetch("/target/wasm32-unknown-unknown/release/js.wasm"), imports).then(v=>{
  let {memory, malloc, main, call0, call1, call2} = v.instance.exports;
  Object.defineProperty(mod,"mem",{get:()=> memory.buffer});
  mod.alloc = malloc;
  mod.call0 = call0;
  mod.call1 = call1;
  mod.call2 = call2;
  main();
});

// })();

