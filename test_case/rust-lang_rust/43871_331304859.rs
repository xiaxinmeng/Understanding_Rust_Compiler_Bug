
error[E0532]: expected tuple struct/variant, found enum `Result`
  --> src/service/prime.rs:18:16
   |
18 |         if let Result(parsed_number) = request.parse::<u64>() {
   |                ^^^^^^ not a tuple struct/variant
   = note: did you mean to use one of the following variants?
           - `Ok(T)`
           - `Err(E)`
