rust
#[entry]
fn main() -> ! {

                    loop {
        delay(1000);
      }
}


fn delay(ms: u16) {
          ms + 1;
}
