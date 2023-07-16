bash
while read t; do cargo +nightly test "$t"; if [ $? != 0 ]; then echo "$t" >> still_failing.txt; fi; done < failures.txt
