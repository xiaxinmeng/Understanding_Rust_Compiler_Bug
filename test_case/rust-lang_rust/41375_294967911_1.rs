c
#include <assert.h>
#include <stdint.h>

typedef struct { int32_t data[5]; } Big;
typedef struct { int32_t data; } Small;

void test(Big a, Big b, Big c, Big d, Big e, Big f, Small g) {
    assert(g.data == 42);
}
