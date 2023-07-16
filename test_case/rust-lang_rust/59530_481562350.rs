
check - par off:
	rustc 47.663s
	rustc_mir 18.725s
	total 0:03:55

check - par on - 1 thread:
	rustc 49.490s
	rustc_mir 19.494s
	total 0:04:00

check - par on - 8 threads:
	rustc 19.847s
	rustc_mir 8.665s
	total 0:02:48
