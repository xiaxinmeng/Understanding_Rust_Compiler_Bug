C
#include <unistd.h>
#include <fcntl.h>
#include <stdio.h>
#include <string.h>

int main() {
  int flags = fcntl(0, F_GETFL, 0);
  fcntl(0, F_SETFL, flags | O_NONBLOCK);
  char c[5] = {0, 0, 0, 0, 0};
  char *msg = "Enter characters: ";
  write(1, msg, strlen(msg));
  while (1) {
    int r = read(0, &c, 5);
    if (r > 0) {
      printf("Read %d characters\n", r);
      write(1, msg, strlen(msg));
    }
  }
  return 0;
}
