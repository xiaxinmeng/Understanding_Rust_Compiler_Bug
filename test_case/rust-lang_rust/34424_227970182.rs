 bash
mergerollup() {
 echo "Merging $@"
 git branch -D current
 git fetch origin pull/$2/head:current
 NL="

"
 A=$(wget -O - https://api.github.com/repos/rust-lang/rust/pulls/$2 -q| grep "\"body\":"| sed  's/^  "body": "//'|sed 's/",$//'| sed 's/\\n/\n/g' | sed 's/\\r//g')

 git merge --no-ff current -m "Rollup merge of #$2 - $1, r=$3 $NL $A";
 git mergetool
}
