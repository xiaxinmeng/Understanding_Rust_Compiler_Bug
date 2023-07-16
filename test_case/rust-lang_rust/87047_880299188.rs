rust
let results = [Ok(1), Err(false), Ok(3), Ok(4), Err(true)];
let mut errs = vec![];
let it = IntoIterator::into_iter(results).error_collector(&mut errs);
let oks = it.collect::<Vec<_>>();
assert_eq!(oks, [1, 3, 4]);
assert_eq!(errs, [false, true]);
