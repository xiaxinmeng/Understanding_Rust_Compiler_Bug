text
$ objdump -d nightly-2019-04-04-x86_64-unknown-linux-gnu/lib/*.so | grep -c 'ud2'
77021

$ objdump -d nightly-2019-04-05-x86_64-unknown-linux-gnu/lib/*.so | grep -c 'ud2'
77636
