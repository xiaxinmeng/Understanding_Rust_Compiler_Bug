
mod intrinsics {
  #[abi = "rust-intrinsic"]
  pub extern "rust-intrinsic" {
      fn sqrtf64(x: f64) -> f64;
  }
}

mod assembly {
  pub fn sqrtf64(x: f64) -> f64 {
    let mut r = 0.0f64;
    unsafe { asm!("sqrtsd $0, $1" : "=x"(r) : "x"(x)); }
    r
  }
}

fn main() {
  let mut s_a: f64 = 0.0;
  let mut s_i: f64 = 0.0;

  for core::uint::range(0, 1000000000) |i| {
    s_i += unsafe { intrinsics::sqrtf64(i as f64) };
    s_a += unsafe { assembly::sqrtf64(i as f64) };
  }

  io::println(fmt!("Intrinsics: %?, Assembly: %?", s_i, s_a));
}
