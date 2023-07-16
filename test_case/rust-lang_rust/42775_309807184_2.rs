c++
// find_if

template <class _InputIterator, class _Predicate>
inline _LIBCPP_INLINE_VISIBILITY
_InputIterator
find_if(_InputIterator __first, _InputIterator __last, _Predicate __pred)
{
    for (; __first != __last; ++__first)
        if (__pred(*__first))
            break;
    return __first;
}
