shell
$ grep 'pgocount = load' $(find target -name '*.ll') | wc -l
199
$ grep 'pgocount = load' $(find target_with_O1 -name '*.ll') | wc -l
169
