
$ time aws --no-sign-request s3 ls static-rust-lang-org/dist/ | wc -l
61253

real	0m45.545s
user	0m21.830s
sys	0m1.977s

$ time aws --no-sign-request s3 ls static-rust-lang-org/dist/2019-09-02/ | wc -l
1998

real	0m4.342s
user	0m1.495s
sys	0m0.143s
