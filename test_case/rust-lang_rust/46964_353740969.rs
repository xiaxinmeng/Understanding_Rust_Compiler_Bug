
extern crate hyper;

use hyper::mime::Name;
use hyper::mime::JSON;

pub fn crash(subtype: Name, suffix: Option<Name>) -> bool {
  match (subtype, suffix) {
    (_, Some(_)) => true,
    (JSON, None) => true,
  }
}
