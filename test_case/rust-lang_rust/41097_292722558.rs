c
struct range {
    int start;
    int end;
};

struct option {
    int ret;
    int tag;
};

struct option next(struct range *range) {
    if (range->start < range->end) {
        int n = range->start;
        range->start++;
        struct option option = {n, 1};
        return option;
    } else {
        struct option option = {0, 0};
        return option;
    }
}

int main() {
    struct range range = {0, 100};
    for(;;) {
        struct option option = next(&range);
        if (option.tag) {
        } else {
            break;
        }
    }
    return 0;
}
