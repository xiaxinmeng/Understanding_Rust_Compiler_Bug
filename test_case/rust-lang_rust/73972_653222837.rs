
2020-07-02T21:04:56.1997387Z [0m[1m[38;5;9merror[0m[0m[1m: this arithmetic operation will overflow[0m
2020-07-02T21:04:56.1998277Z [0m    [0m[0m[1m[38;5;12m--> [0m[0msrc/tools/cargo/src/cargo/core/compiler/fingerprint.rs:1967:21[0m
2020-07-02T21:04:56.1998864Z [0m     [0m[0m[1m[38;5;12m|[0m
2020-07-02T21:04:56.2002872Z [0m[1m[38;5;12m1967[0m[0m [0m[0m[1m[38;5;12m| [0m[0m            assert!(val >> 32 == 0);[0m
2020-07-02T21:04:56.2004266Z [0m     [0m[0m[1m[38;5;12m| [0m[0m                    [0m[0m[1m[38;5;9m^^^^^^^^^[0m[0m [0m[0m[1m[38;5;9mattempt to shift right by 32_i32 which would overflow[0m
2020-07-02T21:04:56.2004924Z [0m     [0m[0m[1m[38;5;12m|[0m
2020-07-02T21:04:56.2005638Z [0m     [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: `#[deny(arithmetic_overflow)]` on by default[0m
2020-07-02T21:04:56.2005849Z 
2020-07-02T21:04:57.0469347Z [0m[1m[38;5;9merror[0m[0m[1m: aborting due to previous error[0m
