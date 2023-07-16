rust
// this condition produces quite some instruction that llvm then need to try optimize. 
let discriminent = if (niche in [begin...end]) { niche - offset } else { data_variant };
match(discriminent) {
   data_variant => ...,
   other_variant1 => ...
   other_variant2 => ...
}
