
error[E0391]: cycle detected when const-evaluating + checking `needs_zst::{{constant}}#0`
  --> src/lib.rs:13:52
   |
13 | fn needs_zst<T>(q: T) where T: IsZst<ZST=ZstHelper<{true}>> {}
   |                                                    ^^^^^^
   |
note: ...which requires const-evaluating + checking `needs_zst::{{constant}}#0`...
  --> src/lib.rs:13:52
   |
13 | fn needs_zst<T>(q: T) where T: IsZst<ZST=ZstHelper<{true}>> {}
   |                                                    ^^^^^^
note: ...which requires const-evaluating `needs_zst::{{constant}}#0`...
  --> src/lib.rs:13:52
   |
13 | fn needs_zst<T>(q: T) where T: IsZst<ZST=ZstHelper<{true}>> {}
   |                                                    ^^^^^^
note: ...which requires processing `needs_zst::{{constant}}#0`...
  --> src/lib.rs:13:52
   |
13 | fn needs_zst<T>(q: T) where T: IsZst<ZST=ZstHelper<{true}>> {}
   |                                                    ^^^^^^
note: ...which requires processing `needs_zst::{{constant}}#0`...
  --> src/lib.rs:13:52
   |
13 | fn needs_zst<T>(q: T) where T: IsZst<ZST=ZstHelper<{true}>> {}
   |                                                    ^^^^^^
   = note: ...which again requires const-evaluating + checking `needs_zst::{{constant}}#0`, completing the cycle
note: cycle used when processing `needs_zst`
  --> src/lib.rs:13:1
   |
13 | fn needs_zst<T>(q: T) where T: IsZst<ZST=ZstHelper<{true}>> {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
