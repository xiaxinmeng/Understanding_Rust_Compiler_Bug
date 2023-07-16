
15:48 <brson> All of the mutexes in Rust are based on a C++ type defined in src/rt/sync/lock_and_signal.h
15:48 <brson> The Rust bindings for this is called LittleLock, in std::unstable::sync
15:49 <brson> the problem is that LittleLock must call into C++ to allocate the lock_and_signal type
15:50 <brson> but many mutexs need to be globally-allocated as `statics`, initialized statically to the unlocked state
15:50 <brson> so none of our locks can be declared `static`
15:51 <brson> this bug is about creating completely new mutex bindings for Rust
15:51 <brson> it will involve creating a correct declaration of the unix pthread_mutex_t as well as the equivalent type on windows
15:52 <brson> meaning, define `struct pthread_muxet_t` so that it has the same definition as in C
15:52 <brson> then creating bindings to all the appropriate pthreads functions that operate on mutexes
15:52 <brson> pthread_mutex_t may have a platform-specific definitions, so you'll need to look up how it is defined in the header files for OS X, FreeBSD, and Linux
15:53 <brson> then, once you have bindings for the unix mutex type and the Windows mutex type, create some abstraction called e.g. `NativeMutex` that delegates to them
15:54 <brson> then start replacing uses of LittleLock with NativeMutex
