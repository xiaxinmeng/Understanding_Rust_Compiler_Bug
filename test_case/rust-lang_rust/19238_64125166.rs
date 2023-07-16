 c
#include <stdio.h>
#include <assert.h>
#include <stdint.h>
#include <unistd.h>

enum { BUFLEN = 1024 * 1024 };

static inline uint64_t rdtsc(uint64_t *id) {
    uint64_t lo, hi;
    __asm__ __volatile__ ( "rdtscp" : "=a"(lo), "=d"(hi), "=c"(*id) : :  );
    return (hi << 32) | lo;
}

int main() {
    int i, diff;
    char buf[BUFLEN];

    uint64_t t0, t1, id0, id1;
    for (i = 0; i < 10; ++i) {
        t0 = rdtsc(&id0);
        if ( ! getcwd(buf, sizeof buf)) {
            perror("getcwd(3)");
            return 10;
        }
        t1 = rdtsc(&id1);

        assert(id0 == id1); /* run with taskset(1) */
    }

    printf("cwd==[%s]\n", buf);
    diff = t1 - t0;
    printf("%.3g cycles; %.3g usec\n", (double)diff, diff / 3.4e3);

    return 0;
}
