
  let s:tempname = tempname()
  exec "!rustc - -o " . s:tempname
  exec "%!" . s:tempname
  unlet s:tempname
  