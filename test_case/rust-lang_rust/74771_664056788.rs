c
#include <assert.h>
#include <unwind.h>

// causes entry for main to be omitted from __unwind_info
void another() {}

int main() {
    assert (_Unwind_FindEnclosingFunction(main) == main);
    return 0;
}
