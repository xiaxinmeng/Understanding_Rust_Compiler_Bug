 rust
            #[repr(u16)]
            #[derive(Copy, Clone)]
            #[allow(non_camel_case_types)]
            pub enum State { $($fe),* , Finished }

            pub struct MapVisitor<'a> {
                pub value: &'a super::$name,
                pub state: State,
            }

            impl<'a> ::serde::ser::MapVisitor for MapVisitor<'a> {
                fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
                    where S: ::serde::Serializer
                {
                    match self.state {
                        $(State::$fe => {
                            self.state = unsafe { ::std::mem::transmute(self.state as u16 + 1) };
                            Ok(Some(try!(serializer.visit_struct_elt(stringify!($fe), &self.value.$fe))))
                        })*
                        State::Finished => {
                            Ok(None)
                        }
                    }
                }
            }
        }
