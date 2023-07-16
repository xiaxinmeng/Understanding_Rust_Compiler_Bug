
immstack.rs:29:4: 30:1 error: borrowed value does not live long enough
immstack.rs:29     &mut Link(item, ~*stack)
immstack.rs:30 }
immstack.rs:28:71: 30:1 note: borrowed pointer must be valid for the lifetime &'a  as defined on the block at 28:71...
immstack.rs:28 fn push<'a, T>(stack : &'a mut Chain<T>, item : T) -> &'a mut Chain<T> {
immstack.rs:29     &mut Link(item, ~*stack)
immstack.rs:30 }
immstack.rs:28:71: 30:1 note: ...but borrowed value is only valid for the block at 28:71
immstack.rs:28 fn push<'a, T>(stack : &'a mut Chain<T>, item : T) -> &'a mut Chain<T> {
immstack.rs:29     &mut Link(item, ~*stack)
immstack.rs:30 }
immstack.rs:29:21: 29:27 error: cannot move out of dereference of & pointer
immstack.rs:29     &mut Link(item, ~*stack)
                                    ^~~~~~
immstack.rs:34:8: 34:30 error: cannot move out of dereference of & pointer
immstack.rs:34         Link(item, ~new_stack) => {
                       ^~~~~~~~~~~~~~~~~~~~~~
immstack.rs:34:19: 34:29 error: cannot move out of dereference of & pointer
immstack.rs:34         Link(item, ~new_stack) => {
                                  ^~~~~~~~~~
immstack.rs:35:20: 35:34 error: borrowed value does not live long enough
immstack.rs:35             stack = &mut new_stack;
                                   ^~~~~~~~~~~~~~
immstack.rs:32:46: 40:1 note: borrowed pointer must be valid for the anonymous lifetime #1 defined on the block at 32:46...
immstack.rs:32 fn pop<T>(stack : &mut Chain<T>) -> Option<T> {
immstack.rs:33     match *stack {
immstack.rs:34         Link(item, ~new_stack) => {
immstack.rs:35             stack = &mut new_stack;
immstack.rs:36             return Some(item);
immstack.rs:37         },
               ...
immstack.rs:33:4: 39:5 note: ...but borrowed value is only valid for the match at 33:4
immstack.rs:33     match *stack {
immstack.rs:34         Link(item, ~new_stack) => {
immstack.rs:35             stack = &mut new_stack;
immstack.rs:36             return Some(item);
immstack.rs:37         },
immstack.rs:38         Break => return None
               ...
immstack.rs:35:20: 35:34 error: cannot borrow immutable local variable as mutable
immstack.rs:35             stack = &mut new_stack;
                                   ^~~~~~~~~~~~~~
immstack.rs:35:12: 35:17 error: cannot assign to immutable argument
immstack.rs:35             stack = &mut new_stack;
                           ^~~~~
immstack.rs:43:4: 43:14 error: cannot borrow immutable static item as mutable
immstack.rs:43     &mut Break
                   ^~~~~~~~~~
immstack.rs:48:19: 48:20 error: cannot borrow immutable dereference of & pointer as mutable
immstack.rs:48     push(push(push(b, 1), 2), 3);
                                  ^
error: aborting due to 9 previous errors
