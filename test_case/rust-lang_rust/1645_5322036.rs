
struct registers_t {
    char data[22];
} __attribute__((aligned(16)));

struct foo { struct registers_t regs; };

void bar(struct foo *s) {} 

int main() {}
