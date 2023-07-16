rust
#[repr(C)] struct S(u8, u32);

#[repr(transparent)]
union U { _x: S, y: () }

unsafe extern "C" fn foo(u: U) {
    let pad = (&u as *const _ as *const u8).add(1).read();
    assert_eq!(pad, 13); // Should fail when called below
}

pub unsafe fn bar() {
  let mut u: U = U { y: () };
  (&mut u as *mut _ as *mut u8).add(1).write(42);
  foo(u);
}
