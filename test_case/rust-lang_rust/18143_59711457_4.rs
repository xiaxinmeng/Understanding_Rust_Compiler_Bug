 C
void black(char *);

long len;
char *data;

void test(void) {
    char *end = data + len;
    char *cur = end;
    for (;;) {
        while (*cur != '>')
            cur = cur - 1;

        black(cur);
    }

}
