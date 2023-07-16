 text
$ git grep "PatUniq"
src/grammar/parser-lalr.y:| BOX pat                                         { $$ = mk_node("PatUniq", 1, $2); }
$
