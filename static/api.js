
(()=>{

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

  // static
  get_str(i, j) {
    return utf8.dec(new Uint8Array(this.mem,i,j));
  },
  put_str(s) {
    let u8 = utf8.enc(s);
    let l = u8.byteLength;
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
      heap = heap[ptr];
      heap[ptr] = e;
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
  log: (i)=> log(i),
  href: ()=> mod.put_str("哈哈哈")
};

const dom = {
  get_body: ()=> heap.add(document.body),
  new_div: ()=> heap.add(document.createElement("div")),
  append: (i, j)=> heap[i].append(heap[j]),
  text: (i, j, k)=> heap[i].textContent = mod.get_str(j, k)
};


// initing
let imports = {
  base, dom
};

WebAssembly.instantiateStreaming(fetch("/target/wasm32-unknown-unknown/release/js.wasm"), imports).then(v=>{
  let {memory, malloc, main, trigger_fn0, trigger_fn1} = v.instance.exports;
  Object.defineProperty(mod,"mem",{get:()=> memory.buffer});
  mod.alloc = malloc;
  main();
  // trigger_fn0(0);
  // trigger_fn1(0,20);
});

})();

