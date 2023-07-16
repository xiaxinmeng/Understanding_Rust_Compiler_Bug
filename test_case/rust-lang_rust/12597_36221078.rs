
$ curl -s http://static.rust-lang.org/doc/master/std/search-index.js | wc -c
501867
$ curl -s http://static.rust-lang.org/doc/master/std/search-index.js | gzip -c -9 - | wc -c
84310
