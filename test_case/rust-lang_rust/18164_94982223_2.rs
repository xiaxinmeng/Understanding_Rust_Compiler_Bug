 rust
pub struct I2CReg {
  test: u32
}

extern {
  #[link_name="k20_iomem_I2C0"] pub static I2C0: I2CReg;
}

struct I2C {
  reg: &'static I2CReg
}

static mut i2c0: I2C = I2C {
  reg: &I2C0
};

fn main() {
    // Add code here
}
