sh
$ hyperfine -L mode off,thin-local -s "x clean; x build library --config config.{mode}.toml" "x t tests/ui --force-rerun --config config.{mode}.toml"
Benchmark 1: x t tests/ui --force-rerun --config config.off.toml
  Time (mean ± σ):     192.653 s ±  7.313 s    [User: 926.090 s, System: 301.098 s]
  Range (min … max):   186.157 s … 200.573 s    3 runs
 
Benchmark 2: x t tests/ui --force-rerun --config config.thin-local.toml
  Time (mean ± σ):     190.043 s ±  4.840 s    [User: 852.896 s, System: 298.910 s]
  Range (min … max):   185.453 s … 195.100 s    3 runs
 
Summary
  'x t tests/ui --force-rerun --config config.thin-local.toml' ran
    1.01 ± 0.05 times faster than 'x t tests/ui --force-rerun --config config.off.toml'
