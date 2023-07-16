 cpp
void SetLinkage(LLVMValue V, unsigned RustLinkage) {
    switch (RustLinkage) {
        case 0:
            SetLLVMLinkage(v, LLVMExternalLinkage);
            break;
        // ...
    }
}
