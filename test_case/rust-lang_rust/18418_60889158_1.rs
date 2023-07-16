
<anon>:18:5: 18:10 error: overflow evaluating the trait `core::kinds::Sized` for the type `<generic #65>`
<anon>:18     print(pair(1));
              ^~~~~
<anon>:18:5: 18:10 note: the trait `core::kinds::Sized` must be implemented because it is required by `print`
<anon>:18     print(pair(1));
              ^~~~~
<anon>:18:5: 18:10 error: overflow evaluating the trait `Printable` for the type `<generic #65>`
<anon>:18     print(pair(1));
              ^~~~~
<anon>:18:5: 18:10 note: the trait `Printable` must be implemented because it is required by `print`
<anon>:18     print(pair(1));
              ^~~~~
<anon>:18:11: 18:18 error: mismatched types: expected `Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<Pair<<generic #65>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`, found `Pair<<generic integer #0>>` (expected struct Pair, found integral variable)
<anon>:18     print(pair(1));
                    ^~~~~~~
error: aborting due to 3 previous errors
playpen: application terminated with error code 101
