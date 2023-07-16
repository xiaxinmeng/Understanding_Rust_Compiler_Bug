 c
#include <pthread.h>
#include <stdio.h>

int main() {
    int err = 0;
    pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;

    err = pthread_mutex_lock(&mutex);
    printf("lock: %d\n", err);

    err = pthread_mutex_unlock(&mutex);
    printf("unlock: %d\n", err);

    err = pthread_mutex_destroy(&mutex);
    printf("destroy1: %d\n", err);

    err = pthread_mutex_destroy(&mutex);
    printf("destroy2: %d\n", err);      

    return 0;
}
