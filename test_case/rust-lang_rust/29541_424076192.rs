text
error: Could not compile `rustc-serialize`.
warning: build failed, waiting for other jobs to finish...
error[E0592]: duplicate definitions with name `item`
    --> C:\Users\Steve Klabnik\.cargo\registry\src\github.com-1ecc6299db9ec823\winapi-0.2.4\src\macros.rs:134:13
     |
134  |                pub unsafe fn $variant(&self) -> &$fieldtype {
     |   _____________-
     |  |_____________|
     | ||
135  | ||                 ::std::mem::transmute(&self.$field)
136  | ||             }
     | ||             -
     | ||_____________|
     | |______________duplicate definitions for `item`
     |                other definition for `item`
     |
    ::: C:\Users\Steve Klabnik\.cargo\registry\src\github.com-1ecc6299db9ec823\winapi-0.2.4\src\commctrl.rs:2889:1
     |
2889 |    UNION!(TVINSERTSTRUCTA, itemex, item, item_mut, TV_ITEMA);
     |    ---------------------------------------------------------- in this macro invocation

error[E0592]: duplicate definitions with name `item_mut`
    --> C:\Users\Steve Klabnik\.cargo\registry\src\github.com-1ecc6299db9ec823\winapi-0.2.4\src\macros.rs:138:13
     |
138  |                pub unsafe fn $variantmut(&mut self) -> &mut $fieldtype {
     |   _____________-
     |  |_____________|
     | ||
139  | ||                 ::std::mem::transmute(&mut self.$field)
140  | ||             }
     | ||             -
     | ||_____________|
     | |______________duplicate definitions for `item_mut`
     |                other definition for `item_mut`
     |
    ::: C:\Users\Steve Klabnik\.cargo\registry\src\github.com-1ecc6299db9ec823\winapi-0.2.4\src\commctrl.rs:2889:1
     |
2889 |    UNION!(TVINSERTSTRUCTA, itemex, item, item_mut, TV_ITEMA);
     |    ---------------------------------------------------------- in this macro invocation

error: aborting due to 2 previous errors
