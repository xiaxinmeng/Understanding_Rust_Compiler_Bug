 rust
if_cfg!(test {
   ...
} else {
  ....
})

if_cfg!(target_word_size="64" { } else { })
if_cfg!(not(test) { ... })
