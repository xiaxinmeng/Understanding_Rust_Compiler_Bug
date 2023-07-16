
2020-07-06T20:42:36.3326114Z [0m[1m[33mwarning[0m[0m[1m: dropping unsupported crate type `dylib` for target `wasm32-wasi`[0m
2020-07-06T20:42:36.3326344Z 
2020-07-06T20:42:37.7271369Z [0m[1m[38;5;9merror[E0308][0m[0m[1m: mismatched types[0m
2020-07-06T20:42:37.7281839Z [0m  [0m[0m[1m[38;5;12m--> [0m[0msrc/libstd/sys/wasi/ext/fs.rs:64:32[0m
2020-07-06T20:42:37.7282253Z [0m   [0m[0m[1m[38;5;12m|[0m
2020-07-06T20:42:37.7282727Z [0m[1m[38;5;12m64[0m[0m [0m[0m[1m[38;5;12m| [0m[0m            match self.read_at(buf, offset) {[0m
2020-07-06T20:42:37.7284275Z [0m   [0m[0m[1m[38;5;12m| [0m[0m                               [0m[0m[1m[38;5;9m^^^[0m[0m [0m[0m[1m[38;5;9mexpected struct `io::IoSliceMut`, found `u8`[0m
2020-07-06T20:42:37.7284677Z [0m   [0m[0m[1m[38;5;12m|[0m
2020-07-06T20:42:37.7285517Z [0m   [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: expected mutable reference `[0m[0m[1m&mut [io::IoSliceMut<'_>][0m[0m`[0m
2020-07-06T20:42:37.7285970Z [0m              found mutable reference `[0m[0m[1m&mut [u8][0m[0m`[0m
2020-07-06T20:42:37.7286100Z 
2020-07-06T20:42:37.7380412Z [0m[1m[38;5;9merror[E0308][0m[0m[1m: mismatched types[0m
2020-07-06T20:42:37.7381049Z [0m   [0m[0m[1m[38;5;12m--> [0m[0msrc/libstd/sys/wasi/ext/fs.rs:124:33[0m
2020-07-06T20:42:37.7381402Z [0m    [0m[0m[1m[38;5;12m|[0m
2020-07-06T20:42:37.7381813Z [0m[1m[38;5;12m124[0m[0m [0m[0m[1m[38;5;12m| [0m[0m            match self.write_at(buf, offset) {[0m
2020-07-06T20:42:37.7382717Z [0m    [0m[0m[1m[38;5;12m| [0m[0m                                [0m[0m[1m[38;5;9m^^^[0m[0m [0m[0m[1m[38;5;9mexpected struct `io::IoSlice`, found `u8`[0m
2020-07-06T20:42:37.7383051Z [0m    [0m[0m[1m[38;5;12m|[0m
2020-07-06T20:42:37.7383731Z [0m    [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: expected reference `[0m[0m[1m&[io::IoSlice<'_>][0m[0m`[0m
2020-07-06T20:42:37.7384102Z [0m               found reference `[0m[0m[1m&[u8][0m[0m`[0m
2020-07-06T20:42:37.7384212Z 
2020-07-06T20:42:37.8993146Z [0m[1m[38;5;9merror[0m[0m[1m: aborting due to 2 previous errors; 1 warning emitted[0m
2020-07-06T20:42:37.8993383Z 
