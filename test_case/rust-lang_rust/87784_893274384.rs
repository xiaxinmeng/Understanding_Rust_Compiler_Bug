
# With fix from #87761 for the failing tests.
./x.py clean; time ./x.py test --stage 1

# no checks:
real	23m19.492s
user	176m22.251s
sys	20m28.837s

# w/ overflow checks in everything but std:
real	23m26.973s
user	176m50.005s
sys	20m32.393s

# w/ overflow checks in everything:
real	23m57.156s
user	181m20.771s
sys	21m25.673s
