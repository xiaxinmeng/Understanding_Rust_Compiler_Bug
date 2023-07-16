
$ curl https://doc.rust-lang.org/search-index1.50.0.js --header "Accept-Encoding: gzip" -o search-index.js.gz
$ zcat search-index.js.gz | brotli > search-index.js.br
$ ls -l search-index.js.*
-rw-rw-r-- 1 jsha jsha 140170 Feb 18 23:56 search-index.js.br
-rw-rw-r-- 1 jsha jsha 272247 Feb 18 23:56 search-index.js.gz
