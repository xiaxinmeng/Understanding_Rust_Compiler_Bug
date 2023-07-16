c
__try {
    RaiseException(MS_VC_EXCEPTION, 0, sizeof(info) / sizeof(ULONG_PTR), (ULONG_PTR*)&info);  
} __except (EXCEPTION_CONTINUE_EXECUTION) {}
