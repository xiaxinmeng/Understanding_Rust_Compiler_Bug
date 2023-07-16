

2020-08-03T23:56:25.9509951Z failures:
2020-08-03T23:56:25.9510092Z 
2020-08-03T23:56:25.9510692Z ---- [mir-opt] mir-opt/instrument_coverage.rs stdout ----
2020-08-03T23:56:25.9511025Z 2	+ // MIR for `main` after InstrumentCoverage
2020-08-03T23:56:25.9511314Z 3	  
2020-08-03T23:56:25.9525434Z 4	  fn main() -> () {
2020-08-03T23:56:25.9526624Z -	      let mut _0: ();                      // return place in scope 0 at /the/cwd/src/test/mir-opt/instrument_coverage.rs:9:11: 9:11
2020-08-03T23:56:25.9527551Z -	      let mut _1: ();                      // in scope 0 at /the/cwd/src/test/mir-opt/instrument_coverage.rs:9:1: 15:2
2020-08-03T23:56:25.9528426Z -	      let mut _2: bool;                    // in scope 0 at /the/cwd/src/test/mir-opt/instrument_coverage.rs:11:12: 11:17
2020-08-03T23:56:25.9529257Z -	      let mut _3: !;                       // in scope 0 at /the/cwd/src/test/mir-opt/instrument_coverage.rs:11:18: 13:10
2020-08-03T23:56:25.9530383Z -	+     let mut _4: ();                      // in scope 0 at /the/cwd/src/test/mir-opt/instrument_coverage.rs:9:11: 9:11
2020-08-03T23:56:25.9530808Z +	      let mut _0: ();                      // return place in scope 0 at $DIR/instrument_coverage.rs:9:11: 9:11
2020-08-03T23:56:25.9531210Z +	      let mut _1: ();                      // in scope 0 at $DIR/instrument_coverage.rs:9:1: 15:2
2020-08-03T23:56:25.9531587Z +	      let mut _2: bool;                    // in scope 0 at $DIR/instrument_coverage.rs:11:12: 11:17
2020-08-03T23:56:25.9531963Z +	      let mut _3: !;                       // in scope 0 at $DIR/instrument_coverage.rs:11:18: 13:10
2020-08-03T23:56:25.9532524Z +	+     let mut _4: ();                      // in scope 0 at $DIR/instrument_coverage.rs:9:11: 9:11
2020
