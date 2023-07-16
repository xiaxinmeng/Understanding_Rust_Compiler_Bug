
wink@3900x 22-08-15T00:05:12.879Z:~/prgs/rust/myrepos/exper-code-coverage (main)
$ cat -n src/if_short_circuit_and.rs 
     1  pub fn if_short_circuit_and(a: i32, b: i32, c: i32, d: i32) -> bool {
     2      if a < b {
     3          c < d
     4      } else {
     5          false
     6      }
     7  }
     8
     9  #[cfg(test)]
    10  mod tests {
    11      use super::*;
    12
    13      // llvm-cov 100% if all are enabled.
    14
    15      // llvm-cov 85.71% if only this is enabled.
    16      #[test]
    17      pub fn test_short_circuit_and_first_false() {
    18          assert_eq!(if_short_circuit_and(20, 10, 30, 40), false);
    19      }
    20
    21      // llvm-cov 85.71% if only this is enabled
    22      #[test]
    23      pub fn test_short_circuit_and_both_true() {
    24          assert_eq!(if_short_circuit_and(10, 20, 30, 40), true);
    25      }
    26  }
wink@3900x 22-08-15T00:05:19.936Z:~/prgs/rust/myrepos/exper-code-coverage (main)
$ cat -n src/short_circuit_and.rs 
     1  pub fn short_circuit_and(a: i32, b: i32, c: i32, d: i32) -> bool {
     2      a < b && c < d
     3  }
     4
     5  #[cfg(test)]
     6  mod tests {
     7      use super::*;
     8
     9      // llvm-cov 100% if all are enabled.
    10
    11      // llvm-cov 83.33% if only this is enabled
    12      #[test]
    13      pub fn test_short_circuit_and_first_false() {
    14          assert_eq!(short_circuit_and(20, 10, 30, 40), false);
    15      }
    16
    17      // llvm-cov 100% if only this is enabled
    18      #[test]
    19      pub fn test_short_circuit_and_both_true() {
    20          assert_eq!(short_circuit_and(10, 20, 30, 40), true);
    21      }
    22  }
wink@3900x 22-08-15T00:05:24.119Z:~/prgs/rust/myrepos/exper-code-coverage (main)
$ cargo llvm-cov
   Compiling exper-code-coverage v0.2.0 (/home/wink/prgs/rust/myrepos/exper-code-coverage)
    Finished test [unoptimized + debuginfo] target(s) in 0.68s
     Running unittests src/lib.rs (target/llvm-cov-target/debug/deps/exper_code_coverage-f4d7ca3a8a720d31)

running 4 tests
test if_short_circuit_and::tests::test_short_circuit_and_both_true ... ok
test short_circuit_and::tests::test_short_circuit_and_both_true ... ok
test if_short_circuit_and::tests::test_short_circuit_and_first_false ... ok
test short_circuit_and::tests::test_short_circuit_and_first_false ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/llvm-cov-target/debug/deps/exper_code_coverage-c502fcb0cc109ca3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/cli.rs (target/llvm-cov-target/debug/deps/cli-55e882f44094d43e)

running 1 test
test test_no_params ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Filename                      Regions    Missed Regions     Cover   Functions  Missed Functions  Executed       Lines      Missed Lines     Cover    Branches   Missed Branches     Cover
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
if_short_circuit_and.rs            10                 0   100.00%           5                 0   100.00%          13                 0   100.00%           0                 0         -
lib.rs                              1                 0   100.00%           1                 0   100.00%           1                 0   100.00%           0                 0         -
main.rs                             2                 0   100.00%           2                 0   100.00%           4                 0   100.00%           0                 0         -
short_circuit_and.rs                9                 0   100.00%           5                 0   100.00%          11                 0   100.00%           0                 0         -
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
TOTAL                              22                 0   100.00%          13                 0   100.00%          29                 0   100.00%           0                 0         -
