 c
#include <dlfcn.h>
#include <pthread.h>
#include <stdio.h>
#include <string.h>

int main()
{
    pthread_attr_t attr;
    pthread_attr_init(&attr);
    memset(&attr, 0, sizeof(attr));
    void *handle = dlopen(NULL, RTLD_LAZY);
    size_t (*__pthread_get_minstack)(const pthread_attr_t *attr) =
        dlsym(handle, "__pthread_get_minstack");
    if (__pthread_get_minstack != NULL)
        printf("%zd\n", __pthread_get_minstack(&attr));
    dlclose(handle);
    pthread_attr_destroy(&attr);
    return 0;
}
