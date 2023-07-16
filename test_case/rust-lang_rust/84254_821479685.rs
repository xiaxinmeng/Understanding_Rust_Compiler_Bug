
$ nm -p /usr/lib/amd64/libc.so.1 /usr/lib/amd64/libgcc_s.so.1 |
    awk '/:$/ { f = $0; sub(":", "", f); sub(".*/", "", f); }
        $NF ~ /^_Unwind/ { c[$NF] += 1; l[$NF] = l[$NF]" "f; }
        END { for (f in c) { printf("%d %s %s\n", c[f], f, l[f]); }}' |
    sort -b -k 1,1n -k 3,3 | column -t

1  _Unwind_ForcedUnwind_Body        libc.so.1
1  _Unwind_RaiseException_Body      libc.so.1
1  _Unwind_Backtrace                libgcc_s.so.1
1  _Unwind_DebugHook                libgcc_s.so.1
1  _Unwind_Find_FDE                 libgcc_s.so.1
1  _Unwind_FindEnclosingFunction    libgcc_s.so.1
1  _Unwind_ForcedUnwind_Phase2      libgcc_s.so.1
1  _Unwind_GetDataRelBase           libgcc_s.so.1
1  _Unwind_GetIPInfo                libgcc_s.so.1
1  _Unwind_GetTextRelBase           libgcc_s.so.1
1  _Unwind_IteratePhdrCallback      libgcc_s.so.1
1  _Unwind_RaiseException_Phase2    libgcc_s.so.1
1  _Unwind_Resume_or_Rethrow        libgcc_s.so.1
2  _Unwind_DeleteException          libc.so.1      libgcc_s.so.1
2  _Unwind_ForcedUnwind             libc.so.1      libgcc_s.so.1
2  _Unwind_GetCFA                   libc.so.1      libgcc_s.so.1
2  _Unwind_GetGR                    libc.so.1      libgcc_s.so.1
2  _Unwind_GetIP                    libc.so.1      libgcc_s.so.1
2  _Unwind_GetLanguageSpecificData  libc.so.1      libgcc_s.so.1
2  _Unwind_GetRegionStart           libc.so.1      libgcc_s.so.1
2  _Unwind_RaiseException           libc.so.1      libgcc_s.so.1
2  _Unwind_Resume                   libc.so.1      libgcc_s.so.1
2  _Unwind_SetGR                    libc.so.1      libgcc_s.so.1
2  _Unwind_SetIP                    libc.so.1      libgcc_s.so.1
