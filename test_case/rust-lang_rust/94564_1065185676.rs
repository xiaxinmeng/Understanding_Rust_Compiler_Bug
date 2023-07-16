
[root@8b906bb3417f /]# cat foo.c
#include <assert.h>
#include <pthread.h>

static pthread_rwlock_t rwl = PTHREAD_RWLOCK_INITIALIZER;

int main() {
  assert(pthread_rwlock_rdlock(&rwl) == 0);
  return 0;
}
[root@8b906bb3417f /]# gcc -static -pthread ./foo.c -o foo
[root@8b906bb3417f /]# ./foo
foo: ./foo.c:7: main: Assertion `pthread_rwlock_rdlock(&rwl) == 0' failed.
Aborted (core dumped)
