
                time      file size(bytes)
5k lines of format!("foo {}", "bar")
before w/o opt  1m3.6s    4853288
after w/o opt   1m9.1s    4853296
before w/t -O   1m23.3s   3231128
after w/t -O    1m24.2s   3231136

2k lines of format! with 10 arguments, 5 unnamed and 5 named
before w/o opt  3m28.2s   7659744
after w/o opt   4m47.5s   8079328
before w/t -O   3m48.6s   3613024
after w/t -O    5m10s     3613024
