rust
struct Led(usize); // the usize is some GPIO pin or I²C address or so

impl Led {
  fn on(&mut self) {
    // send GPIO/I²C messages to turn on LED
  }

  fn off(&mut self) {
    // send GPIO/I²C messages to turn off LED
  }
}
