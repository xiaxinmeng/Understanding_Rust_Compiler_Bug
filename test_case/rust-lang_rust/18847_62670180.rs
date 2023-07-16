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

#define N 200

#define CHECK(e) if (!(e)) {              \
    printf("%s failed: %d\n", #e, errno); \
    perror("failure");                    \
    assert(0);                            \
  }

int main() {
  int s = socket(AF_INET, SOCK_STREAM, 0);
  CHECK(s != -1);
  int opt = 1;
  CHECK(setsockopt(s, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt)) == 0);

  struct sockaddr_in ip4addr;
  ip4addr.sin_family = AF_INET;
  ip4addr.sin_port = htons(8000);
  inet_pton(AF_INET, "127.0.0.1", &ip4addr.sin_addr);
  CHECK(bind(s, (struct sockaddr*) &ip4addr, sizeof(ip4addr)) == 0);
  CHECK(listen(s, 1) == 0);

  while (1) {
    int c = accept(s, NULL, NULL);
    CHECK(c != -1);
    char buf[1];
    switch (read(c, buf, 1)) {
      case 0: printf("eof\n"); break;
      case 1: break;
      default: printf("read error\n"); break;
    }
    CHECK(write(c, "a", 1) == 1);
    close(c);
  }
}
