shell
egrep -rnw ./src/tools/rustfmt/* -e '\(\".*?{}.*?\",( *[a-zA-Z_]+ *,?)+\)'
