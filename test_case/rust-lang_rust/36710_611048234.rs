c++
extern "C" void *__dso_handle = 0;
 
extern "C" int __cxa_atexit(void (*destructor) (void *), void *arg, void *dso) {};
extern "C" void __cxa_finalize(void *f) {};
