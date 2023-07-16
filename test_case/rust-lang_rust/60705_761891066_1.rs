
$ nm libregex-old.rlib | grep -o "__Z.*$" | wc -c
822521
$ nm libregex-new.rlib | grep -o "__R.*$" | wc -c
1156947
