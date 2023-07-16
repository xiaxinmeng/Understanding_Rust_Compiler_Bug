
#![cfg_attr(debug_assertions, feature(try_from))]

#[cfg(debug_assertions)]
use std::convert::TryFrom;

#[cfg(not(debug_assertions))]
macro_rules! to { ($e:expr, $t:ident) => (($e) as $t)  }

#[cfg(debug_assertions)]
macro_rules! to { ($e:expr, $t:ident) => ($t::try_from($e).unwrap())  }
