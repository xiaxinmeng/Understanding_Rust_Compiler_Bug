
# good: [509f29ac17874394acf4d49d6bae3cd93c652aa1] [WebAssembly] Fix fast-isel optimization of branch conditions.
git bisect good 509f29ac17874394acf4d49d6bae3cd93c652aa1
# bad: [03684905101f0b7e49dfe530e54dc1aeac6ef0fb] Fix compile on dist-x86_64-linux builder
git bisect bad 03684905101f0b7e49dfe530e54dc1aeac6ef0fb
# bad: [2c3c78e685eeaed179014d9d5fc6e279ca28b70a] Make ICF log output order deterministic.
git bisect bad 2c3c78e685eeaed179014d9d5fc6e279ca28b70a
# bad: [1b838dc9f8531990d7d8fcaa0abd144ce0407f29] Remove left-over debug printout from r321692
git bisect bad 1b838dc9f8531990d7d8fcaa0abd144ce0407f29

