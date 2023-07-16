c
#include <stdio.h>
#include <assert.h>
#include <pthread.h>
#include <sys/types.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

#define N 4

static pthread_t threads[N];

static void spawn() {
  pid_t p;
  p = fork();
  assert(p != -1);

  if (p == 0) {
    _exit(0);
  } else {
    pid_t r;
    int wstatus;
    r = waitpid(p, &wstatus, 0);
    assert(r != -1);
    assert(r == p);
  }
}

static void *run(void *arg) {
  for (int i=0; i != 10; ++i) {
    spawn();
  }
  return NULL;
}

int main() {
  int r;
  for (int i=0; i != N; ++i) {
    r = pthread_create(threads +i, NULL, run, NULL);
    assert(r == 0);
  }
  for (int i=0; i != N; ++i) {
    r = pthread_join(threads[i], NULL);
    assert(r == 0);
  }
  return 0;
}
