
int *a, *b;
for (int i = 0; i < 4; i++) {
    a[i & 1] = b[i & 1];
}
