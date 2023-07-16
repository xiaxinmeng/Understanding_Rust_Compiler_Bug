c
void bad1(bool *x) {
    if (*x) {
        *x = *x;
    }
}

void bad2(bool *x) {
    if (*x) {
        *x = false;
        *x = true;
    }
}

void bad3(bool *x) {
    if (!*x) {
        *x = *x;
    }
}
