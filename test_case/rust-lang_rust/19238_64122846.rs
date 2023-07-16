
$ p=`printf "test%0248ddir" 0`; for k in {1..20}; do mkdir $p && cd $p; done
$ pwd | wc
      1       1    5134
