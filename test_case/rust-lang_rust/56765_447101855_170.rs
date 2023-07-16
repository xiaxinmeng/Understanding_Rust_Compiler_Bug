\n\nNote that this approach needs a reference to S with lifetime `'a`.\nNothing shorter than `'a` will suffice: a shorter lifetime would imply\nthat after `demo` finishes executing, something else (such as the\ndestructor!) could access `s.data` after the end of that shorter\nlifetime, which would again violate the `&mut`-borrow's exclusive\naccess.\n"},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-45696-scribble-on-boxed-borrow.rs","byte_start":3214,"byte_end":3215,"line_start":76,"line_end":76,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"here, drop of `s` needs exclusive access to `*s.0`, because the type `Scribble<'_>` implements the `Drop` trait","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-45696-scribble-on-boxed-borrow.rs","byte_start":2903,"byte_end":2905,"line_start":72,"line_end":72,"column_start":20,"column_end":22,"is_primary":false,"text":[{"text":"fn boxed_scribbled<'a>(s: Box<Scribble<'a>>) -> &'a mut u32 {","highlight_start":20,"highlight_end":22}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-45696-scribble-on-boxed-borrow.rs","byte_start":2950,"byte_end":2962,"line_start":73,"line_end":73,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    &mut *(*s).0 //[nll]~ ERROR borrow may still be in use when destructor runs [E0713]","highlight_start":5,"highlight_end":17}],"label":"returning this value requires that `*s.0` is borrowed for `'a`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"This error has been downgraded to a warning for backwards compatibility with previous releases.\nI    let mut long_lived = Scribble(&mut x);","highlight_start":1,"highlight_end":47},{"text":"        *borrowed_scribble(&mut long_lived) += 10;","highlight_start":1,"highlight_end":51},{"text":"        // (Scribble dtor runs here, after `&mut`-borrow above ends)","highlight_start":1,"highlight_end":69},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"    {","highlight_start":1,"highlight_end":6},{"text":"        let mut long_lived = Scribble(&mut x);","highlight_start":1,"highlight_end":47},{"text":"        *boxed_borrowed_scribble(Box::new(&mut long_lived)) += 10;","highlight_start":1,"highlight_end":67},{"text":"        // (Scribble dtor runs here, after `&mut`-borrow above ends)","highlight_start":1,"highlight_end":69},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"    {","highlight_start":1,"highlight_end":6},{"text":"        let mut long_lived = Scribble(&mut x);","highlight_start":1,"highlight_end":47},{"text":"        *boxed_boxed_borrowed_scribble(Box::new(Box::new(&mut long_lived))) += 10;","highlight_start":1,"highlight_end":83},{"text":"        // (Scribble dtor runs here, after `&mut`-borrow above ends)","highlight_start":1,"highlight_end":69},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"    *scribbled(Scribble(&mut x)) += 10;","highlight_start":1,"highlight_end":40},{"text":"    *boxed_scribbled(Box::new(Scribble(&mut x))) += 10;","highlight_start":1,"highlight_end":56},{"text":"    *boxed_boxed_scribbled(Box::new(Box::new(Scribble(&mut x)))) += 10;","highlight_start":1,"highlight_end":72},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggeste] diff of stderr:
[01:00:57] 
[01:00:57] - error[E0017]: references in statics may only refer to immutable values
[01:00:57] -    |
[01:00:57] -    |
[01:00:57] - LL | static buf: &mut [u8] = &mut [1u8,2,3,4,5,7];   //[ast]~ ERROR E0017
[01:00:57] -    |                         ^^^^^^^^^^^^^^^^^^^^ statics require immutable values
[01:00:57] - 
[01:00:57] 7 error[E0594]: cannot assign to `buf[..]`, as `buf` is an immutable static item
[01:00:57] 9    |
[01:00:57] 
[01:00:57] 
[01:00:57] 10 LL |     buf[0]=2;                                   //[ast]~ ERROR E0389
[01:00:57] + 
[01:00:57] + 
[01:00:57] + error[E0017]: references in statics may only refer to immutable values
[01:00:57] +   --> $DIR/issue-46604.rs:14:25
[01:00:57] +    |
[01:00:57] + LL | static buf: &mut [u8] = &mut [1u8,2,3,4,5,7];   //[ast]~ ERROR E0017
[01:00:57] +    |                         ^^^^^^^^^^^^^^^^^^^^ statics require immutable values
[01:00:57] 13 error: aborting due to 2 previous errors
[01:00:57] 14 
[01:00:57] 
[01:00:57] 
[01:00:57] 
[01:00:57] The actual stderr differed from the expected stderr.
[01:00:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46604.mir/issue-46604.mir.stderr
[01:00:57] To update references, rerun the tests and pass the `--bless` flag
[01:00:57] To only update this specific test, also pass `--test-args issues/issue-46604.rs`
[01:00:57] 
[01:00:57] error in revision `mir`: 1 errors occurred comparing output.
[01:00:57] status: exit code: 1
[01:00:57] c:"E0593","explanation":"\nYou tried to supply an `Fn`-based type with an incorrect number of arguments\nthan what was expected.\n\nErroneous code example:\n\n