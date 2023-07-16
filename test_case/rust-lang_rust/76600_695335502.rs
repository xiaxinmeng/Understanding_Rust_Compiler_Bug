c
#include <assert.h>
#include <pthread.h>
#include <stdlib.h>

#define N 4

static pthread_t threads[N];

static void *run(void *arg) {
        return malloc(1024);
}

int main() {
        for (int i = 0; i != N; ++i) assert(pthread_create(&threads[i], NULL, run, NULL) == 0);
        for (int i = 0; i != N; ++i) assert(pthread_join(threads[i], NULL) == 0);
}
