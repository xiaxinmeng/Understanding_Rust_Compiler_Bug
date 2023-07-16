
auto merge of #20935 : dotdash/rust/cpu_x86-64, r=luqmana

Using "generic" disables a number of features that are present on all
x86_64 cpus, the "x86-64" target cpu is the common denominator for that
arch.

Refs #20777
