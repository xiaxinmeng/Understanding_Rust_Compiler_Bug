
[INFO] [stderr] error: internal compiler error: broken MIR in DefId(0:521 ~ rand[bbc7]::prng[0]::chacha[0]::{{impl}}[2]::reseed[0]) (Terminator { source_info: SourceInfo { span: /opt/rustwide/cargo-home/registry/src/github.com-1ecc6299db9ec823/rand-0.4.6/src/prng/chacha.rs:205:9: 205:38, scope: scope[0] }, kind: _3 = const prng::chacha::ChaChaRng::init(move _4, move _5) -> [return: bb2, unwind: bb1] }): bad arg #1 (&[u32; 8] <- &[u32; _]): NoSolution
[INFO] [stderr]    --> /opt/rustwide/cargo-home/registry/src/github.com-1ecc6299db9ec823/rand-0.4.6/src/prng/chacha.rs:205:19
[INFO] [stderr]     |
[INFO] [stderr] 205 |         self.init(&[0u32; KEY_WORDS]);
[INFO] [stderr]     |                   ^^^^^^^^^^^^^^^^^^
[INFO] [stderr] 
[INFO] [stderr] thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:360:17
