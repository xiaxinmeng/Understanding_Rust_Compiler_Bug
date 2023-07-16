
#[derive(Clone, Copy)]
pub struct Something;
impl Something {
  pub const fn copy(&self) -> Self {
      *self
  }
}

pub const CLOSURE: &'static dyn Fn(&mut u8) = &(|_| ());
const BASE : Something = Something{};

const COPY : (&'static dyn Fn(&mut u8), &'static Something) = (CLOSURE, &(BASE.copy()) );
