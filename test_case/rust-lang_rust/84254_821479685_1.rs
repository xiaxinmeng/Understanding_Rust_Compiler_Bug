
$ nm -p /usr/lib/amd64/libc.so.1 /usr/lib/amd64/libgcc_s.so.1 | awk '/:$/ { f = $0; sub(":", "", f); sub(".*/", "", f); } $NF ~ /^_Unwind/ { c[$NF] += 1; l[$NF] = l[$NF]" "f; } END { for (f in c) { printf("%d %s %s\n", c[f], f, l[f]); }}' | sort -b -k 1,1n -k 3,3 | column -t | awk '$1 == 1 && $3 == "libgcc_s.so.1" { print $2 }' > /tmp/pats.txt

$ rg -Io -Ff /tmp/pats.txt /ws/rust/{library,compiler} | sort -u

_Unwind_Backtrace
_Unwind_FindEnclosingFunction
_Unwind_GetDataRelBase
_Unwind_GetIPInfo
_Unwind_GetTextRelBase
