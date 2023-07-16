 c
#include <pthread.h>
#include <stdlib.h>
#include <stdio.h>
#include <errno.h>

void *cb(void *arg) {
        pthread_exit(0);
        return NULL;
}

int main(void) {
        void *retval;
        int ret;

        pthread_t *threads = malloc(sizeof(pthread_t) * 100000);
        for (int i = 0; i < 100000; i++) {
                if ((ret = pthread_create(&threads[i], NULL, cb, NULL)) != 0) {
                        printf("failed on thread %d\n", i);
                        errno = ret;
                        perror("pthread_create");
                        exit(1);
                }
        }

        for (int i = 0; i < 100000; i++) {
                pthread_join(threads[i], retval);
        }

        return 0;
}
