 rust
macro_rules! e_class(
    ($S_name:ident,
     $( $method_name:ident $enum_name:ident ),*)
    =>
    {
        pub impl $S_name {
            pub fn name(index:uint) -> &'static str {
                match index {
                    0 => "nil", 1 => "lft", 2 => "rgt", 3 => "all",
                    4 => "end", _ => "???"
                }
            }

            $( pub fn $method_name (&self) -> int { self.items[$enum_name as uint] } )*
        }
    }
)

e_class!( S,
          nil_item Enil ,
          rgt_item Ergt ,
          lft_item Elft ,
          all_item Eall ,
          end_item Eend )

