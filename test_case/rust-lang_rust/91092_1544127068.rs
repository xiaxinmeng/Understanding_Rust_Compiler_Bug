console
$ rustc -C llvm-args='--help-list-hidden' | rg 'atomic-counter'
  --atomic-counter-update-promoted                                  - Do counter update using atomic fetch add  for promoted counters only
  --gcov-atomic-counter                                             - Make counter updates atomic
  --instrprof-atomic-counter-update-all                             - Make all profile counter updates atomic (for testing only)
