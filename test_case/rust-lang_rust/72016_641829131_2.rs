rust
  #![feature(asm)]

  use core::arch::x86_64::*;

  #[inline]
  fn secure(mut x: f64) -> f64 {
      unsafe {
          asm!("# {0}", inout(xmm_reg) x, options(nomem, nostack));
      }
      x
  }

  #[inline]
  fn mul(x: f64, y: f64) -> f64 {
      secure(secure(x) * secure(y))
  }

  fn main() {
      unsafe {
          _MM_SET_ROUNDING_MODE(_MM_ROUND_UP);
      }

      assert_ne!(-mul(-1.1, 10.1), mul(1.1, 10.1));

      unsafe {
          _MM_SET_ROUNDING_MODE(_MM_ROUND_NEAREST);
      }
  }
  