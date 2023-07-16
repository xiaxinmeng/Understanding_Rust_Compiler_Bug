 c
#include <sys/types.h>
#include <arpa/inet.h>
#include <netinet/in.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netdb.h>
#include <stdio.h>
#include <errno.h>
#include <pthread.h>
#include <assert.h>

#define N 10

#define CHECK(e) if (!(e)) {              \
    printf("%s failed: %d\n", #e, errno); \
    perror("failure");                    \
    assert(0);                            \
  }

void *child(void *foo) {
  int s = socket(AF_INET, SOCK_STREAM, 0);
  CHECK(s != -1);

  struct sockaddr_in ip4addr;
  ip4addr.sin_family = AF_INET;
  ip4addr.sin_port = htons(8000);
  inet_pton(AF_INET, "127.0.0.1", &ip4addr.sin_addr);
  CHECK(connect(s, (struct sockaddr*) &ip4addr, sizeof(ip4addr)) == 0);

  CHECK(write(s, "GET / HTTP/1.0\r\n", 16) == 16);
  CHECK(write(s, "\r\n", 2) == 2);
  char buf[1];
  CHECK(read(s, buf, 1) == 1);
  close(s);

  return foo;
}

int main() {
  pthread_t children[N];

  int i;
  for (i = 0; i < N; i++) {
    CHECK(pthread_create(&children[i], NULL, child, NULL) == 0);
  }
  for (i = 0; i < N; i++) {
    CHECK(pthread_join(children[i], NULL) == 0);
  }
}
