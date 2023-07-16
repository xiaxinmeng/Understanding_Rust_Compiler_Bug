 rust
mod foo {
    pub struct Bar { ... }
    #[macro_export_or_whatever]
    macro_rules! bar_update_xy {
        ($x:expr, $y:expr, $b:expr) =>
          { Bar { x: $x, y: $y,
            other_1: $b.other_1,
            [... manually list other fields here ...]
            other_k: $b.other_k } }
    }
}

let b1: Bar = ...;
let b2 : Bar = bar_update_xy!(new_x(), new_y(), b1));
