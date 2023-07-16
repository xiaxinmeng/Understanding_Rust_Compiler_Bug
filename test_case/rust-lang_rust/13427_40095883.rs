 c
#include <assert.h>
#include <fcntl.h>
#include <pthread.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

static pthread_mutex_t lock;
static pthread_cond_t cond;
static volatile int cnt = 0;

static void test() {
  int p = fork();
  assert(p >= 0);
  if (p == 0) {
    _exit(0);
  }
  int a = 1;
  assert(waitpid(p, &a, 0) == p);
  assert(WEXITSTATUS(a) == 0);
}

static void *child(void *arg) {
  test();

  assert(pthread_mutex_lock(&lock) == 0);
  cnt -= 1;
  if (cnt == 0) {
    assert(pthread_cond_signal(&cond) == 0);
  }
  assert(pthread_mutex_unlock(&lock) == 0);
  return arg;
}

int main() {
  assert(pthread_mutex_init(&lock, NULL) == 0);
  assert(pthread_cond_init(&cond, NULL) == 0);

  pthread_t c1, c2;
  cnt = 2;
  assert(pthread_create(&c1, NULL, child, NULL) == 0);
  assert(pthread_create(&c2, NULL, child, NULL) == 0);

  assert(pthread_mutex_lock(&lock) == 0);
  while (cnt != 0) {
    assert(pthread_cond_wait(&cond, &lock) == 0);
  }
  assert(pthread_mutex_unlock(&lock) == 0);

  pthread_join(c1, NULL);
  pthread_join(c2, NULL);
}
