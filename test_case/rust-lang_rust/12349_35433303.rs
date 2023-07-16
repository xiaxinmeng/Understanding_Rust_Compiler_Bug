
after-1 = naive match expression
after-2 = one match nested level every 8 arguments

                time      file size(bytes)
5k lines of format!("foo {}", "bar")
before w/o opt  1m3.6s    4853288
after-1 w/o opt 1m9.1s    4853296
after-2 w/o opt 1m10s     5041808
before w/t -O   1m23.3s   3231128
after-1 w/t -O  1m24.2s   3231136
after-2 w/t -O  1m26s     3231136

2k lines of format! with 10 arguments, 5 unnamed and 5 named
before w/o opt  3m28.2s   7659744
after-1 w/o opt 4m47.5s   8079328
after-2 w/o opt 3m36s     8349816
before w/t -O   3m48.6s   3613024
after-1 w/t -O  5m10s     3613024
after-2 w/t -O  3m49s     3613024
