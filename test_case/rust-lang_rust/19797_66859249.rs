 c
#include <assert.h>
#include <pthread.h>
#include <stdio.h>

int main() {
    pthread_mutex_t lock;

    assert(pthread_mutex_init(&lock, NULL) == 0);
    assert(pthread_mutex_trylock(&lock) == 0);
    assert(pthread_mutex_unlock(&lock) == 0);
    assert(pthread_mutex_destroy(&lock) == 0);
}
