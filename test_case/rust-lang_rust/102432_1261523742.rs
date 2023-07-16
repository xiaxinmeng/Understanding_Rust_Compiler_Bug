plain
error: unnecessary braces around block return value
    --> compiler/rustc_span/src/hygiene.rs:1333:5
     |
1333 | /     {
1334 | |         if let Some(ctxt) = outer_ctxts.lock().get(raw_id as usize).copied().flatten() {
1335 |               return ctxt;
1336 |           }
     |  __________^
1337 | |     }
1337 | |     }
     | |_____^
     |
     = note: `-D unused-braces` implied by `-D warnings`
help: remove these braces
     |
1333 ~     if let Some(ctxt) = outer_ctxts.lock().get(raw_id as usize).copied().flatten() {
1334 |             return ctxt;
     |

error: could not compile `rustc_span` due to previous error
warning: build failed, waiting for other jobs to finish...
