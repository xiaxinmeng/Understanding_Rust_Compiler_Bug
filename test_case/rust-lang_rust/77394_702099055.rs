
# Whether or not to optimize the compiler and standard library.
# WARNING: Building with optimize = false is NOT SUPPORTED. Due to bootstrapping,
# building without optimizations takes much longer than optimizing. Further, some platforms
# fail to build without this optimization (c.f. #65352).
optimize = false
