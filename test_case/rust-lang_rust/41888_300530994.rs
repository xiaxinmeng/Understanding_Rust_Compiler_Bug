rust
(lldb) bt
* thread #1, queue = 'com.apple.main-thread', stop reason = EXC_BAD_ACCESS (code=1, address=0x0)
  * frame #0: 0x0000000100000e39 1`core::ptr::drop_in_place<1::Expression>((null)=0x0000000000000000) at ptr.rs:61 [opt]
    frame #1: 0x0000000100000e72 1`core::ptr::drop_in_place<1::Expression> [inlined] core::ptr::drop_in_place<alloc::boxed::Box<1::Expression>> at ptr.rs:61 [opt]
    frame #2: 0x0000000100000e69 1`core::ptr::drop_in_place<1::Expression> [inlined] core::ptr::drop_in_place<1::Binary> at ptr.rs:61 [opt]
    frame #3: 0x0000000100000e69 1`core::ptr::drop_in_place<1::Expression>((null)=0x00007fff5fbff470) at ptr.rs:61 [opt]
    frame #4: 0x0000000100001672 1`_ZN114mainE [inlined] core::ptr::drop_in_place<1::InfixOrPostfix> at ptr.rs:61 [opt]
    frame #5: 0x0000000100001666 1`_ZN114mainE at 1.rs:273 [opt]
    frame #6: 0x0000000100000ef4 1`_ZN114mainE [inlined] _ZN1110expressionE at 1.rs:236 [opt]
    frame #7: 0x0000000100000ef4 1`_ZN114mainE at 1.rs:5 [opt]
    frame #8: 0x000000010000b45c 1`panic_abort::__rust_maybe_catch_panic at lib.rs:40 [opt]
    frame #9: 0x000000010000b172 1`std::rt::lang_start [inlined] std::panicking::try<(),fn()> at panicking.rs:433 [opt]
    frame #10: 0x000000010000b143 1`std::rt::lang_start [inlined] std::panic::catch_unwind<fn(),()> at panic.rs:361 [opt]
    frame #11: 0x000000010000b143 1`std::rt::lang_start at rt.rs:57 [opt]
    frame #12: 0x00007fffc988b235 libdyld.dylib`start + 1
    frame #13: 0x00007fffc988b235 libdyld.dylib`start + 1

(lldb) fr s 7
frame #7: 0x0000000100000ef4 1`_ZN114mainE at 1.rs:5 [opt]
   2   	
   3   	fn main() {
   4   	    let tokens = &[Token::Ident, Token::AmpersandEquals, Token::Ident];
-> 5   	    let _ = expression(Point::new(tokens));
   6   	}
   7   	
   8   	struct Progress<'s, T> {

(lldb) fr s 6
frame #6: 0x0000000100000ef4 1`_ZN114mainE [inlined] _ZN1110expressionE at 1.rs:236 [opt]
   233 	}
   234 	
   235 	fn expression<'s>(pt: Point<'s>) -> Progress<'s, Expression> {
-> 236 	    match expression_x(pt) {
   237 	        Ok(ShuntCar { value: expr, ept, .. }) => Progress::success(ept, expr),
   238 	        Err(failure_point) => Progress::failure(failure_point),
   239 	    }

(lldb) fr s 5
frame #5: 0x0000000100001666 1`_ZN114mainE at 1.rs:273 [opt]
   270 	                        pt = point;
   271 	                    }
   272 	                    _ => shunting_yard.apply_all()?,
-> 273 	                }
   274 	            }
   275 	        }
   276 	    }

(lldb) fr s 4
frame #4: 0x0000000100001672 1`_ZN114mainE [inlined] core::ptr::drop_in_place<1::InfixOrPostfix> at ptr.rs:61 [opt]

(lldb) fr s 3
frame #3: 0x0000000100000e69 1`core::ptr::drop_in_place<1::Expression>((null)=0x00007fff5fbff470) at ptr.rs:61 [opt]

(lldb) fr s 2
frame #2: 0x0000000100000e69 1`core::ptr::drop_in_place<1::Expression> [inlined] core::ptr::drop_in_place<1::Binary> at ptr.rs:61 [opt]

(lldb) fr s 1
frame #1: 0x0000000100000e72 1`core::ptr::drop_in_place<1::Expression> [inlined] core::ptr::drop_in_place<alloc::boxed::Box<1::Expression>> at ptr.rs:61 [opt]

(lldb) fr s 0
frame #0: 0x0000000100000e39 1`core::ptr::drop_in_place<1::Expression>((null)=0x0000000000000000) at ptr.rs:61 [opt]
