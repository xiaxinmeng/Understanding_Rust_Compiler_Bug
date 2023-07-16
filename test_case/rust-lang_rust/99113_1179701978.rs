plain
   Compiling rustc-demangle v0.1.21
error: unnecessary parentheses around block return value
    --> library/alloc/src/rc.rs:2699:5
     |
2699 |     (layout.size() + layout.padding_needed_for(align))
     |     ^                                                ^
     |
     = note: `-D unused-parens` implied by `-D warnings`
help: remove these parentheses
     |
2699 -     (layout.size() + layout.padding_needed_for(align))
2699 +     layout.size() + layout.padding_needed_for(align)

error: unnecessary parentheses around block return value
    --> library/alloc/src/sync.rs:2762:5
     |
     |
2762 |     (layout.size() + layout.padding_needed_for(align))
     |     ^                                                ^
help: remove these parentheses
     |
     |
2762 -     (layout.size() + layout.padding_needed_for(align))
2762 +     layout.size() + layout.padding_needed_for(align)

error: could not compile `alloc` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:00:17
