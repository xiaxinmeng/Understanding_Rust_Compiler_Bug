c
#include <dlfcn.h>
#include <stdio.h>

int main() {
    printf("running\n");
    void* handle = dlopen("./libtest.dylib", RTLD_LAZY);
    printf("opened: %p\n", handle);

    int (*test_fn)() = dlsym(handle, "test_fn");
    printf("test_fn: %d\n", test_fn());

    printf("Closing...\n");
    int code = dlclose(handle); // Removing this line prevents the segfault upon exit?
    printf("Closed: %d.\n", code);
}
