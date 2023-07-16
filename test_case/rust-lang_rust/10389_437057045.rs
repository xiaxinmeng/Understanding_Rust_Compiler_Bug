cpp
template <typename T>
struct VtableForAny {
  std::size_t size;
  std::size_t align;

  void (*destroy)(void *);
};
template <typename T>
inline VtableForAny<T> VTABLE_FOR_ANY = {
  sizeof(T),
  alignof(T),
  T::~T(),
};

struct Foo {
};

template <typename T>
T *downcast(AnyRef any) {
  if (any.vtable == &VTABLE_FOR_ANY<T>) {
    return static_cast<T*>(any.ptr);
  } else {
    return nullptr;
  }
}
