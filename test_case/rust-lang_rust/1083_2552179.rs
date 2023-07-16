 rust
use std;
import std::float;
fn main() {
  let nan = float::NaN();
  let inf = float::infinity();
  assert(-inf == float::neg_infinity());

  assert(!( nan >  nan));
  assert(!( nan > -nan));
  assert(!( nan >   0.));
  assert(!( nan >  inf));
  assert(!( nan > -inf));
  assert(!(  0. >  nan));
  assert(!( inf >  nan));
  assert(!(-inf >  nan));
  assert(!(-nan >  nan));

  assert( nan ==  nan);
  assert( nan == -nan);

  assert( nan ==   1.);
  assert( nan ==   0.);
  assert( nan ==  inf);
  assert( nan == -inf);

  assert(  1. ==  nan);
  assert(  0. ==  nan);
  assert( inf ==  nan);
  assert(-inf ==  nan);

  assert(nan <   0.);
  assert(nan <   1.);
  assert(nan <  -1.);
  assert(nan <  inf);
  assert(nan < -inf);
  assert(nan <  nan);
  assert(nan < -nan);

  assert(  0. < nan);
  assert(  1. < nan);
  assert( -1. < nan);
  assert( inf < nan);
  assert(-inf < nan);
  assert(-nan < nan);
}
