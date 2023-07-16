
struct ThreadContext {};


void gt_switch(struct ThreadContext *new) {
        asm(
        "mov     0x0(%0), %%rsp\n\t"
        "ret\n"
        : 
        : "*m"(new)
        : "rsp"
        );
}

int main() {
    struct ThreadContext ctx;

    gt_switch(&ctx);
    return 0;
}
