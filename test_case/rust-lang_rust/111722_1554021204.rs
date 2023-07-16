
To check if stack smashing protection is enabled for a given binary, search
for cross references to `__stack_chk_fail`. The only cross references to
`__stack_chk_fail` in hello-rust are from the statically-linked libbacktrace
library (see Fig. 14).
