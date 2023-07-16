
$ clang++ -fsyntax-only -Werror=exit-time-destructors -std=c++14 -stdlib=libc++ -xc++ - <<<'
#include <mutex>

std::recursive_mutex *getManagedStaticMutex() {
  static std::recursive_mutex m;
  return &m;
}
'
<stdin>:5:31: error: declaration requires an exit-time destructor [-Werror,-Wexit-time-destructors]
  static std::recursive_mutex m;
                              ^
1 error generated.
