
$ hyperfine -L commit 87745f71312bb8459c1ab728c3ae76f8d0f50801,bda32a4023b1d3f96e56e1b2fc7510324f430316 -s "git checkout --force {commit} && git submodule update" -p "x clean" -w 1 -m 3 -n "Commit {commit}: `x help`" "x help || true"

Benchmark 1: Commit 87745f71312bb8459c1ab728c3ae76f8d0f50801: `x help`
  Time (mean ± σ):     34.434 s ±  0.189 s    [User: 168.235 s, System: 11.999 s]
  Range (min … max):   34.243 s … 34.620 s    3 runs

Benchmark 2: Commit bda32a4023b1d3f96e56e1b2fc7510324f430316: `x help`
  Time (mean ± σ):     36.572 s ±  0.236 s    [User: 163.442 s, System: 11.890 s]
  Range (min … max):   36.343 s … 36.814 s    3 runs

Summary
  'Commit 87745f71312bb8459c1ab728c3ae76f8d0f50801: `x help`' ran
    1.06 ± 0.01 times faster than 'Commit bda32a4023b1d3f96e56e1b2fc7510324f430316: `x help`'
