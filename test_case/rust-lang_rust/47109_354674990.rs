c
HANDLE_DI_FLAG(0, Zero) // Use it as zero value.
                        // For example: void foo(DIFlags Flags = FlagZero).
HANDLE_DI_FLAG(1, Private)
HANDLE_DI_FLAG(2, Protected)
HANDLE_DI_FLAG(3, Public)
HANDLE_DI_FLAG((1 << 2), FwdDecl)
