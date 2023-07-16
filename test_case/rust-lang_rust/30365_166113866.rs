
failures:

---- [run-pass-valgrind] run-pass-valgrind/osx-frameworks.rs stdout ----

error: test run failed!
status: exit code: 100
command: /usr/local/bin/valgrind --error-exitcode=100 --fair-sched=try --quiet --soname-synonyms=somalloc=NONE --suppressions=/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/etc/x86.supp --suppressions=/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/etc/apple-darwin.supp --tool=memcheck --leak-check=full x86_64-apple-darwin/test/run-pass-valgrind/osx-frameworks.stage2-x86_64-apple-darwin
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
==42057== 1,040 bytes in 1 blocks are possibly lost in loss record 337 of 352
==42057==    at 0x6DDC: malloc_zone_malloc (in /usr/local/Cellar/valgrind/3.10.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==42057==    by 0xA5C293: NXCreateMapTableFromZone (in /usr/lib/libobjc.A.dylib)
==42057==    by 0xA6FD40: nonMetaClasses() (in /usr/lib/libobjc.A.dylib)
==42057==    by 0xA61537: objc_registerClassPair (in /usr/lib/libobjc.A.dylib)
==42057==    by 0x96A68: __CFMakeNSBlockClasses (in /System/Library/Frameworks/CoreFoundation.framework/Versions/A/CoreFoundation)
==42057==    by 0x7E98E: __CFInitialize (in /System/Library/Frameworks/CoreFoundation.framework/Versions/A/CoreFoundation)
==42057==    by 0x7FFF5FC13255: ImageLoaderMachO::doImageInit(ImageLoader::LinkContext const&) (in /usr/lib/dyld)
==42057==    by 0x7FFF5FC13756: ImageLoaderMachO::doInitialization(ImageLoader::LinkContext const&) (in /usr/lib/dyld)
==42057==    by 0x7FFF5FC1006D: ImageLoader::recursiveInitialization(ImageLoader::LinkContext const&, unsigned int, ImageLoader::InitializerTimingList&) (in /usr/lib/dyld)
==42057==    by 0x7FFF5FC0FFC3: ImageLoader::recursiveInitialization(ImageLoader::LinkContext const&, unsigned int, ImageLoader::InitializerTimingList&) (in /usr/lib/dyld)
==42057==    by 0x7FFF5FC0FEB9: ImageLoader::runInitializers(ImageLoader::LinkContext const&, ImageLoader::InitializerTimingList&) (in /usr/lib/dyld)
==42057==    by 0x7FFF5FC01FBF: dyld::initializeMainExecutable() (in /usr/lib/dyld)
==42057== 

------------------------------------------

thread '[run-pass-valgrind] run-pass-valgrind/osx-frameworks.rs' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/compiletest/runtest.rs:1505
