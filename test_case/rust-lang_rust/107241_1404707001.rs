
$ hyperfine -L mode off,thinlocal -p "x clean; x build library --dry-run" "x build library --config config.{mode}.toml";hyperfine -L mode off,thinlocal -p "x clean; x build library; touch library/core/src/lib.rs" "x build library --config config.{mode}.toml"

# Clean build
Benchmark 1: x build library --config config.off.toml
  Time (mean ± σ):     25.388 s ±  0.361 s    [User: 59.866 s, System: 5.066 s]
  Range (min … max):   25.094 s … 26.181 s    10 runs

Benchmark 2: x build library --config config.thinlocal.toml
  Time (mean ± σ):     26.330 s ±  0.308 s    [User: 68.647 s, System: 6.686 s]
  Range (min … max):   26.096 s … 27.065 s    10 runs

Summary
  'x build library --config config.off.toml' ran
    1.04 ± 0.02 times faster than 'x build library --config config.thinlocal.toml'

# Frech incr comp (touch library/core/src/lib.rs)
Benchmark 1: x build library --config config.off.toml
  Time (mean ± σ):     11.516 s ±  0.159 s    [User: 20.368 s, System: 3.478 s]
  Range (min … max):   11.392 s … 11.936 s    10 runs

Benchmark 2: x build library --config config.thinlocal.toml
  Time (mean ± σ):     25.453 s ±  0.412 s    [User: 65.687 s, System: 5.908 s]
  Range (min … max):   25.105 s … 26.519 s    10 runs

Summary
  'x build library --config config.off.toml' ran
    2.21 ± 0.05 times faster than 'x build library --config config.thinlocal.toml'
