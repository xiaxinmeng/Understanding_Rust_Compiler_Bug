c
int main() {
    int start = 0;
    int end = 10000;
    int tag = 0;
    do {
        tag = 0;
        if (start < end) start++, tag = 1;
    } while (tag);
    return 0;
}
