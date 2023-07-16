c
#include <assert.h>

enum foo
{
    BAR
};


int main(void)
{
    static_assert(sizeof(enum foo) == 2);
}
