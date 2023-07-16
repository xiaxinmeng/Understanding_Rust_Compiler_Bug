
const foo::{{initializer}}: usize ={
  let mut _0: usize; // return place

  bb0: { 
  _0 = const 4096usize; // bb0[0]: scope 0 at /tmp/compiler-explorer-compiler118318-63-13s7ff5.cpzhg/example.rs:1:26: 1:30
  return; // bb0[1]: scope 0 at /tmp/compiler-explorer-compiler118318-63-13s7ff5.cpzhg/example.rs:1:26: 1:30
  }
}

const foo::{{initializer}}: usize ={
  let mut _0: usize; // return place

  bb0: { 
  _0 = const 4096usize; // bb0[0]: scope 0 at /tmp/compiler-explorer-compiler118318-63-13s7ff5.cpzhg/example.rs:2:18: 2:22
  return; // bb0[1]: scope 0 at /tmp/compiler-explorer-compiler118318-63-13s7ff5.cpzhg/example.rs:2:18: 2:22
  }
}

fn foo() -> std::boxed::Box<[u8; 4096]>{
  let mut _0: std::boxed::Box<[u8; 4096]>; // return place
  let mut _1: [u8; 4096];

  bb0: { 
  StorageLive(_1); // bb0[0]: scope 0 at /tmp/compiler-explorer-compiler118318-63-13s7ff5.cpzhg/example.rs:2:14: 2:23
  _1 = [const 0u8; 4096]; // bb0[1]: scope 0 at /tmp/compiler-explorer-compiler118318-63-13s7ff5.cpzhg/example.rs:2:14: 2:23
  _0 = const <std::boxed::Box<T>>::new(move _1) -> bb1; // bb0[2]: scope 0 at /tmp/compiler-explorer-compiler118318-63-13s7ff5.cpzhg/example.rs:2:5: 2:24
  }

  bb1: { 
  StorageDead(_1); // bb1[0]: scope 0 at /tmp/compiler-explorer-compiler118318-63-13s7ff5.cpzhg/example.rs:2:23: 2:24
  return; // bb1[1]: scope 0 at /tmp/compiler-explorer-compiler118318-63-13s7ff5.cpzhg/example.rs:3:2: 3:2
  }
}
