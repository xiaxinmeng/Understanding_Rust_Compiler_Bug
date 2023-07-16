rust
[...]
        _38 = const NaNf64;              // scope 0 at src/main.rs:5:32: 5:55
                                         // mir::Constant
                                         // + span: src/main.rs:5:32: 5:55
                                         // + literal: Const { ty: f64, val: Value(Scalar(0x7ff8000000000000)) }
        _37 = f64::<impl f64>::copysign(const 1f64, move _38) -> bb8; // scope 0 at src/main.rs:5:16: 5:56
[...]
