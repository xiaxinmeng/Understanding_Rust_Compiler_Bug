rust
match(niche) {
   other_variant1+offset => ...
   other_variant2+offset => ...
    _ => ...   // (for the data_variant)
   // begin..end  =>  ...    // alternative if the switch is not exhaustive
}
