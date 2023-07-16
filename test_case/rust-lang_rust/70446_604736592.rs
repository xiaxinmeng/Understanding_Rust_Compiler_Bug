rust
macro_rules! m {
          (          $(: $p:path)? : $l:lifetime)
    => {m!{@internal $(: $p     )? : $l         }};

          (@internal $(: $p:path)? : $l:lifetime) => {};
}

m! {: 'static}

fn main() {}
