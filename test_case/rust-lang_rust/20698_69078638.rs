 c
#include <assert.h>
#include <pthread.h>

int main() {
  pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
  assert(pthread_mutex_destroy(&lock) == 0);
}
