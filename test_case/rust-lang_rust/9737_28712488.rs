
-> (cond [else 1] [#t 2])
; readline-input:5:6: cond: bad syntax (`else' clause must be last)
-> (let [(else #f)] (cond [else 1] [#t 2]))
2
