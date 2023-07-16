
asgn_expr
: range_expr
| range_expr "=" asgn_expr
;

range_expr
: logic_or_expr
| logic_or_expr ".." logic_or_expr
| logic_or_expr ".."
| ".." logic_or_expr
| ".."

logic_or_expr
: logic_and_expr
| logic_or_expr "||" logic_and_expr
;
