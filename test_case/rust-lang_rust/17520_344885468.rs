
#if __ANDROID_API__ >= 21
int dl_iterate_phdr(int (*)(struct dl_phdr_info*, size_t, void*), void*) __INTRODUCED_IN(21);
#endif /* __ANDROID_API__ >= 21 */
