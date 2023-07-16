c
#include <assert.h>
#include <pthread.h>
#include <signal.h>
#include <stdlib.h>
#include <sys/types.h>
#include <unistd.h>

void *lazy_abort(void *arg) {
  struct timespec req = {.tv_sec = 0, .tv_nsec = 1000000 };
  nanosleep(&req, NULL);
  abort();
  return NULL;
}

int main() {
  pthread_t thread;
  signal(SIGABRT, SIG_IGN);
  int r = pthread_create(&thread, NULL, lazy_abort, NULL);
  assert(r == 0);

  while (1) {
    pid_t pid = fork();
    assert(pid != -1);
    if (pid == 0) {
      abort();
    }
  }
}
