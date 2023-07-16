
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
fn main() {
              let vec = get_vec();
              match vec {
                  Err(err) => {

                      $crate::io::_print(::std::fmt::Arguments::new_v1({
                                                                           static __STATIC_FMTSTR:
                                                                                  &'static [&'static str]
                                                                                  =
                                                                               &["Got an error: ",
                                                                                 "\n"];
                                                                           __STATIC_FMTSTR
                                                                       },
                                                                       &match (&err,)
                                                                            {
                                                                            (__arg0,)
                                                                            =>
                                                                            [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                         ::std::fmt::Debug::fmt)],
                                                                        }));
                  }
                  _ if vec.unwrap().len() == 0 => {
                      $crate::io::_print(::std::fmt::Arguments::new_v1({
                                                                           static __STATIC_FMTSTR:
                                                                                  &'static [&'static str]
                                                                                  =
                                                                               &["Vec was 0 length.\n"];
                                                                           __STATIC_FMTSTR
                                                                       },
                                                                       &match ()
                                                                            {
                                                                            ()
                                                                            =>
                                                                            [],
                                                                        }));
                  }
