
$ nm target/debug/rg-old | grep -o "__Z.*$" | wc -c
3298005
$ nm target/debug/rg-new | grep -o "__R.*$" | wc -c
5415482
