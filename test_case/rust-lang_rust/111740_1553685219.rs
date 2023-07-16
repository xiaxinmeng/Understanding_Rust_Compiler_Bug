plain
Loaded plugins: fastestmirror, ovl
Determining fastest mirrors
 * base: forksystems.mm.fcix.net
 * extras: mirror.umd.edu
 * updates: mirror.siena.edu
--> Running transaction check
---> Package bash.x86_64 0:4.2.46-34.el7 will be updated
---> Package bash.x86_64 0:4.2.46-35.el7_9 will be an update
---> Package bind-license.noarch 32:9.11.4-26.P2.el7 will be updated
---
Loaded plugins: fastestmirror, ovl
Loading mirror speeds from cached hostfile
 * base: forksystems.mm.fcix.net
 * extras: mirror.umd.edu
 * updates: mirror.siena.edu
--> Running transaction check
---> Package epel-release.noarch 0:7-11 will be installed
--> Finished Dependency Resolution

---
Loading mirror speeds from cached hostfile
 * base: forksystems.mm.fcix.net
 * epel: dl.fedoraproject.org
 * extras: mirror.umd.edu
 * updates: mirror.siena.edu
Trying other mirror.
Package 1:pkgconfig-0.27.1-4.el7.x86_64 already installed and latest version
Package xz-5.2.2-2.el7_9.x86_64 already installed and latest version
Resolving Dependencies
---
In file included from /checkout/src/llvm-project/llvm/include/llvm/ADT/StringMap.h:17:
In file included from /checkout/src/llvm-project/llvm/include/llvm/ADT/StringMapEntry.h:19:
In file included from /checkout/src/llvm-project/llvm/include/llvm/ADT/StringRef.h:12:
In file included from /checkout/src/llvm-project/llvm/include/llvm/ADT/DenseMapInfo.h:23:
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:1559:55: error: '__get' is missing exception specification 'noexcept'
        friend constexpr decltype(auto) __detail::__variant::__get(_Vp&& __v);
                                                             ^
/checkout/src/llvm-project/llvm/utils/TableGen/DFAEmitter.cpp:360:33: note: in instantiation of template class 'std::variant<llvm::Record *, unsigned int, std::basic_string<char>>' requested here
  for (const auto &SingleAction : AT) {
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:263:5: note: previous declaration is here
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:263:5: note: previous declaration is here
    __get(_Variant&& __v) noexcept
    ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:266:38: error: '_M_u' is a private member of 'std::__detail::__variant::_Variant_storage<false, llvm::Record *, unsigned int, std::basic_string<char>>'
                              std::forward<_Variant>(__v)._M_u);
                                                          ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:1116:45: note: in instantiation of function template specialization 'std::__detail::__variant::__get<0UL, const std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &>' requested here
        return std::addressof(__detail::__variant::__get<_Np>(*__ptr));
                                                   ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:1138:19: note: in instantiation of function template specialization 'std::get_if<0UL, llvm::Record *, unsigned int, std::basic_string<char>>' requested here
      return std::get_if<__detail::__variant::__index_of_v<_Tp, _Types...>>(
                  ^
/checkout/src/llvm-project/llvm/utils/TableGen/DFAEmitter.cpp:362:30: note: in instantiation of function template specialization 'std::get_if<llvm::Record *, llvm::Record *, unsigned int, std::basic_string<char>>' requested here
    if (const auto *R = std::get_if<Record *>(&SingleAction))
                             ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:1220:7: note: constrained by private inheritance here
    : private __detail::__variant::_Variant_base<_Types...>,
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:419:34: note: member is declared here
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:419:34: note: member is declared here
      _Variadic_union<_Types...> _M_u;
                                 ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:266:38: error: '_M_u' is a private member of 'std::__detail::__variant::_Variant_storage<false, llvm::Record *, unsigned int, std::basic_string<char>>'
                              std::forward<_Variant>(__v)._M_u);
                                                          ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:1116:45: note: in instantiation of function template specialization 'std::__detail::__variant::__get<2UL, const std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &>' requested here
        return std::addressof(__detail::__variant::__get<_Np>(*__ptr));
                                                   ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:1138:19: note: in instantiation of function template specialization 'std::get_if<2UL, llvm::Record *, unsigned int, std::basic_string<char>>' requested here
      return std::get_if<__detail::__variant::__index_of_v<_Tp, _Types...>>(
                  ^
/checkout/src/llvm-project/llvm/utils/TableGen/DFAEmitter.cpp:364:35: note: in instantiation of function template specialization 'std::get_if<std::basic_string<char>, llvm::Record *, unsigned int, std::basic_string<char>>' requested here
    else if (const auto *S = std::get_if<std::string>(&SingleAction))
                                  ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:1220:7: note: constrained by private inheritance here
    : private __detail::__variant::_Variant_base<_Types...>,
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:419:34: note: member is declared here
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:419:34: note: member is declared here
      _Variadic_union<_Types...> _M_u;
                                 ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:266:38: error: '_M_u' is a private member of 'std::__detail::__variant::_Variant_storage<false, llvm::Record *, unsigned int, std::basic_string<char>>'
                              std::forward<_Variant>(__v)._M_u);
                                                          ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:1610:35: note: in instantiation of function template specialization 'std::__detail::__variant::__get<1UL, const std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &>' requested here
      return __detail::__variant::__get<_Np>(__v);
                                  ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:1080:19: note: in instantiation of function template specialization 'std::get<1UL, llvm::Record *, unsigned int, std::basic_string<char>>' requested here
      return std::get<__detail::__variant::__index_of_v<_Tp, _Types...>>(__v);
                  ^
/checkout/src/llvm-project/llvm/utils/TableGen/DFAEmitter.cpp:367:18: note: in instantiation of function template specialization 'std::get<unsigned int, llvm::Record *, unsigned int, std::basic_string<char>>' requested here
      OS << std::get<unsigned>(SingleAction);
                 ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:1220:7: note: constrained by private inheritance here
    : private __detail::__variant::_Variant_base<_Types...>,
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:419:34: note: member is declared here
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:419:34: note: member is declared here
      _Variadic_union<_Types...> _M_u;
                                 ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:266:38: error: '_M_u' is a private member of 'std::__detail::__variant::_Variant_storage<false, llvm::Record *, unsigned int, std::basic_string<char>>'
                              std::forward<_Variant>(__v)._M_u);
                                                          ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:951:24: note: in instantiation of function template specialization 'std::__detail::__variant::__get<2UL, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &>' requested here
            return __variant::__get<__index>(std::forward<_Variant>(__var));
                              ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:973:8: note: in instantiation of function template specialization 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:385:13) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &>, std::integer_sequence<unsigned long, 2>>::__element_by_index_or_cookie<2UL, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &>' requested here
              __element_by_index_or_cookie<__indices>(
              ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:980:9: note: in instantiation of member function 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:385:13) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &>, std::integer_sequence<unsigned long, 2>>::__visit_invoke_impl' requested here
        return __visit_invoke_impl(std::forward<_Visitor>(__visitor),
               ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:996:11: note: in instantiation of member function 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:385:13) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &>, std::integer_sequence<unsigned long, 2>>::__do_visit_invoke' requested here
          return __do_visit_invoke(std::forward<_Visitor>(__visitor),
                 ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:1005:29: note: in instantiation of member function 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:385:13) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &>, std::integer_sequence<unsigned long, 2>>::__visit_invoke' requested here
      { return _Array_type{&__visit_invoke}; }
                            ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:927:48: note: (skipping 16 contexts in backtrace; use -ftemplate-backtrace-limit=0 to see all)
                std::index_sequence<__indices..., __index>>::_S_apply();
                                                             ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/stl_uninitialized.h:289:19: note: in instantiation of function template specialization 'std::uninitialized_copy<const (anonymous namespace)::Transition *, (anonymous namespace)::Transition *>' requested here
    { return std::uninitialized_copy(__first, __last, __result); }
                  ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/stl_uninitialized.h:310:19: note: in instantiation of function template specialization 'std::__uninitialized_copy_a<const (anonymous namespace)::Transition *, (anonymous namespace)::Transition *, (anonymous namespace)::Transition>' requested here
      return std::__uninitialized_copy_a
                  ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/vector.tcc:473:10: note: in instantiation of function template specialization 'std::__uninitialized_move_if_noexcept_a<(anonymous namespace)::Transition *, (anonymous namespace)::Transition *, std::allocator<(anonymous namespace)::Transition>>' requested here
                = std::__uninitialized_move_if_noexcept_a
                       ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/vector.tcc:121:4: note: in instantiation of function template specialization 'std::vector<(anonymous namespace)::Transition>::_M_realloc_insert<llvm::Record *&, (anonymous namespace)::Automaton *>' requested here
          ^
          ^
/checkout/src/llvm-project/llvm/utils/TableGen/DFAEmitter.cpp:254:17: note: in instantiation of function template specialization 'std::vector<(anonymous namespace)::Transition>::emplace_back<llvm::Record *&, (anonymous namespace)::Automaton *>' requested here
    Transitions.emplace_back(T, this);
                ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:1220:7: note: constrained by private inheritance here
    : private __detail::__variant::_Variant_base<_Types...>,
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:419:34: note: member is declared here
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:419:34: note: member is declared here
      _Variadic_union<_Types...> _M_u;
                                 ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:266:38: error: '_M_u' is a private member of 'std::__detail::__variant::_Variant_storage<false, llvm::Record *, unsigned int, std::basic_string<char>>'
                              std::forward<_Variant>(__v)._M_u);
                                                          ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:951:24: note: in instantiation of function template specialization 'std::__detail::__variant::__get<1UL, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &>' requested here
            return __variant::__get<__index>(std::forward<_Variant>(__var));
                              ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:973:8: note: in instantiation of function template specialization 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:385:13) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &>, std::integer_sequence<unsigned long, 1>>::__element_by_index_or_cookie<1UL, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &>' requested here
              __element_by_index_or_cookie<__indices>(
              ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:980:9: note: in instantiation of member function 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:385:13) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &>, std::integer_sequence<unsigned long, 1>>::__visit_invoke_impl' requested here
        return __visit_invoke_impl(std::forward<_Visitor>(__visitor),
               ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:996:11: note: in instantiation of member function 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:385:13) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &>, std::integer_sequence<unsigned long, 1>>::__do_visit_invoke' requested here
          return __do_visit_invoke(std::forward<_Visitor>(__visitor),
                 ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:1005:29: note: in instantiation of member function 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:385:13) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &>, std::integer_sequence<unsigned long, 1>>::__visit_invoke' requested here
      { return _Array_type{&__visit_invoke}; }
                            ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:927:48: note: (skipping 16 contexts in backtrace; use -ftemplate-backtrace-limit=0 to see all)
                std::index_sequence<__indices..., __index>>::_S_apply();
                                                             ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/stl_uninitialized.h:289:19: note: in instantiation of function template specialization 'std::uninitialized_copy<const (anonymous namespace)::Transition *, (anonymous namespace)::Transition *>' requested here
    { return std::uninitialized_copy(__first, __last, __result); }
                  ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/stl_uninitialized.h:310:19: note: in instantiation of function template specialization 'std::__uninitialized_copy_a<const (anonymous namespace)::Transition *, (anonymous namespace)::Transition *, (anonymous namespace)::Transition>' requested here
      return std::__uninitialized_copy_a
                  ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/vector.tcc:473:10: note: in instantiation of function template specialization 'std::__uninitialized_move_if_noexcept_a<(anonymous namespace)::Transition *, (anonymous namespace)::Transition *, std::allocator<(anonymous namespace)::Transition>>' requested here
                = std::__uninitialized_move_if_noexcept_a
                       ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/vector.tcc:121:4: note: in instantiation of function template specialization 'std::vector<(anonymous namespace)::Transition>::_M_realloc_insert<llvm::Record *&, (anonymous namespace)::Automaton *>' requested here
          ^
          ^
/checkout/src/llvm-project/llvm/utils/TableGen/DFAEmitter.cpp:254:17: note: in instantiation of function template specialization 'std::vector<(anonymous namespace)::Transition>::emplace_back<llvm::Record *&, (anonymous namespace)::Automaton *>' requested here
    Transitions.emplace_back(T, this);
                ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:1220:7: note: constrained by private inheritance here
    : private __detail::__variant::_Variant_base<_Types...>,
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:419:34: note: member is declared here
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:419:34: note: member is declared here
      _Variadic_union<_Types...> _M_u;
                                 ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:266:38: error: '_M_u' is a private member of 'std::__detail::__variant::_Variant_storage<false, llvm::Record *, unsigned int, std::basic_string<char>>'
                              std::forward<_Variant>(__v)._M_u);
                                                          ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:951:24: note: in instantiation of function template specialization 'std::__detail::__variant::__get<0UL, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &>' requested here
            return __variant::__get<__index>(std::forward<_Variant>(__var));
                              ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:973:8: note: in instantiation of function template specialization 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:385:13) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &>, std::integer_sequence<unsigned long, 0>>::__element_by_index_or_cookie<0UL, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &>' requested here
              __element_by_index_or_cookie<__indices>(
              ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:980:9: note: in instantiation of member function 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:385:13) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &>, std::integer_sequence<unsigned long, 0>>::__visit_invoke_impl' requested here
        return __visit_invoke_impl(std::forward<_Visitor>(__visitor),
               ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:996:11: note: in instantiation of member function 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:385:13) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &>, std::integer_sequence<unsigned long, 0>>::__do_visit_invoke' requested here
          return __do_visit_invoke(std::forward<_Visitor>(__visitor),
                 ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:1005:29: note: in instantiation of member function 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:385:13) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &>, std::integer_sequence<unsigned long, 0>>::__visit_invoke' requested here
      { return _Array_type{&__visit_invoke}; }
                            ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:927:48: note: (skipping 16 contexts in backtrace; use -ftemplate-backtrace-limit=0 to see all)
                std::index_sequence<__indices..., __index>>::_S_apply();
                                                             ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/stl_uninitialized.h:289:19: note: in instantiation of function template specialization 'std::uninitialized_copy<const (anonymous namespace)::Transition *, (anonymous namespace)::Transition *>' requested here
    { return std::uninitialized_copy(__first, __last, __result); }
                  ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/stl_uninitialized.h:310:19: note: in instantiation of function template specialization 'std::__uninitialized_copy_a<const (anonymous namespace)::Transition *, (anonymous namespace)::Transition *, (anonymous namespace)::Transition>' requested here
      return std::__uninitialized_copy_a
                  ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/vector.tcc:473:10: note: in instantiation of function template specialization 'std::__uninitialized_move_if_noexcept_a<(anonymous namespace)::Transition *, (anonymous namespace)::Transition *, std::allocator<(anonymous namespace)::Transition>>' requested here
                = std::__uninitialized_move_if_noexcept_a
                       ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/vector.tcc:121:4: note: in instantiation of function template specialization 'std::vector<(anonymous namespace)::Transition>::_M_realloc_insert<llvm::Record *&, (anonymous namespace)::Automaton *>' requested here
          ^
          ^
/checkout/src/llvm-project/llvm/utils/TableGen/DFAEmitter.cpp:254:17: note: in instantiation of function template specialization 'std::vector<(anonymous namespace)::Transition>::emplace_back<llvm::Record *&, (anonymous namespace)::Automaton *>' requested here
    Transitions.emplace_back(T, this);
                ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:1220:7: note: constrained by private inheritance here
    : private __detail::__variant::_Variant_base<_Types...>,
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:419:34: note: member is declared here
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:419:34: note: member is declared here
      _Variadic_union<_Types...> _M_u;
                                 ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:266:38: error: '_M_u' is a private member of 'std::__detail::__variant::_Variant_storage<false, llvm::Record *, unsigned int, std::basic_string<char>>'
                              std::forward<_Variant>(__v)._M_u);
                                                          ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:951:24: note: in instantiation of function template specialization 'std::__detail::__variant::__get<2UL, std::variant<llvm::Record *, unsigned int, std::basic_string<char>>>' requested here
            return __variant::__get<__index>(std::forward<_Variant>(__var));
                              ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:973:8: note: in instantiation of function template specialization 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:476:18) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&>, std::integer_sequence<unsigned long, 2>>::__element_by_index_or_cookie<2UL, std::variant<llvm::Record *, unsigned int, std::basic_string<char>>>' requested here
              __element_by_index_or_cookie<__indices>(
              ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:980:9: note: in instantiation of member function 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:476:18) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&>, std::integer_sequence<unsigned long, 2>>::__visit_invoke_impl' requested here
        return __visit_invoke_impl(std::forward<_Visitor>(__visitor),
               ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:996:11: note: in instantiation of member function 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:476:18) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&>, std::integer_sequence<unsigned long, 2>>::__do_visit_invoke' requested here
          return __do_visit_invoke(std::forward<_Visitor>(__visitor),
                 ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:1005:29: note: in instantiation of member function 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:476:18) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&>, std::integer_sequence<unsigned long, 2>>::__visit_invoke' requested here
      { return _Array_type{&__visit_invoke}; }
                            ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:927:48: note: (skipping 12 contexts in backtrace; use -ftemplate-backtrace-limit=0 to see all)
                std::index_sequence<__indices..., __index>>::_S_apply();
                                                             ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/stl_vector.h:453:14: note: in instantiation of function template specialization 'std::__relocate_a<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> *, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> *, std::allocator<std::variant<llvm::Record *, unsigned int, std::basic_string<char>>>>' requested here
        return std::__relocate_a(__first, __last, __result, __alloc);
                    ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/stl_vector.h:466:9: note: in instantiation of member function 'std::vector<std::variant<llvm::Record *, unsigned int, std::basic_string<char>>>::_S_do_relocate' requested here
        return _S_do_relocate(__first, __last, __result, __alloc, __do_it{});
               ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/vector.tcc:461:23: note: in instantiation of member function 'std::vector<std::variant<llvm::Record *, unsigned int, std::basic_string<char>>>::_S_relocate' requested here
              __new_finish = _S_relocate(__old_start, __position.base(),
                             ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/vector.tcc:121:4: note: in instantiation of function template specialization 'std::vector<std::variant<llvm::Record *, unsigned int, std::basic_string<char>>>::_M_realloc_insert<llvm::Record *>' requested here
          ^
          ^
/checkout/src/llvm-project/llvm/utils/TableGen/DFAEmitter.cpp:324:15: note: in instantiation of function template specialization 'std::vector<std::variant<llvm::Record *, unsigned int, std::basic_string<char>>>::emplace_back<llvm::Record *>' requested here
      Actions.emplace_back(R->getValueAsDef(A));
              ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:1220:7: note: constrained by private inheritance here
    : private __detail::__variant::_Variant_base<_Types...>,
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:419:34: note: member is declared here
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:419:34: note: member is declared here
      _Variadic_union<_Types...> _M_u;
                                 ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:266:38: error: '_M_u' is a private member of 'std::__detail::__variant::_Variant_storage<false, llvm::Record *, unsigned int, std::basic_string<char>>'
                              std::forward<_Variant>(__v)._M_u);
                                                          ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:951:24: note: in instantiation of function template specialization 'std::__detail::__variant::__get<1UL, std::variant<llvm::Record *, unsigned int, std::basic_string<char>>>' requested here
            return __variant::__get<__index>(std::forward<_Variant>(__var));
                              ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:973:8: note: in instantiation of function template specialization 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:476:18) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&>, std::integer_sequence<unsigned long, 1>>::__element_by_index_or_cookie<1UL, std::variant<llvm::Record *, unsigned int, std::basic_string<char>>>' requested here
              __element_by_index_or_cookie<__indices>(
              ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:980:9: note: in instantiation of member function 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:476:18) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&>, std::integer_sequence<unsigned long, 1>>::__visit_invoke_impl' requested here
        return __visit_invoke_impl(std::forward<_Visitor>(__visitor),
               ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:996:11: note: in instantiation of member function 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:476:18) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&>, std::integer_sequence<unsigned long, 1>>::__do_visit_invoke' requested here
          return __do_visit_invoke(std::forward<_Visitor>(__visitor),
                 ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:1005:29: note: in instantiation of member function 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:476:18) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&>, std::integer_sequence<unsigned long, 1>>::__visit_invoke' requested here
      { return _Array_type{&__visit_invoke}; }
                            ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:927:48: note: (skipping 12 contexts in backtrace; use -ftemplate-backtrace-limit=0 to see all)
                std::index_sequence<__indices..., __index>>::_S_apply();
                                                             ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/stl_vector.h:453:14: note: in instantiation of function template specialization 'std::__relocate_a<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> *, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> *, std::allocator<std::variant<llvm::Record *, unsigned int, std::basic_string<char>>>>' requested here
        return std::__relocate_a(__first, __last, __result, __alloc);
                    ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/stl_vector.h:466:9: note: in instantiation of member function 'std::vector<std::variant<llvm::Record *, unsigned int, std::basic_string<char>>>::_S_do_relocate' requested here
        return _S_do_relocate(__first, __last, __result, __alloc, __do_it{});
               ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/vector.tcc:461:23: note: in instantiation of member function 'std::vector<std::variant<llvm::Record *, unsigned int, std::basic_string<char>>>::_S_relocate' requested here
              __new_finish = _S_relocate(__old_start, __position.base(),
                             ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/vector.tcc:121:4: note: in instantiation of function template specialization 'std::vector<std::variant<llvm::Record *, unsigned int, std::basic_string<char>>>::_M_realloc_insert<llvm::Record *>' requested here
          ^
          ^
/checkout/src/llvm-project/llvm/utils/TableGen/DFAEmitter.cpp:324:15: note: in instantiation of function template specialization 'std::vector<std::variant<llvm::Record *, unsigned int, std::basic_string<char>>>::emplace_back<llvm::Record *>' requested here
      Actions.emplace_back(R->getValueAsDef(A));
              ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:1220:7: note: constrained by private inheritance here
    : private __detail::__variant::_Variant_base<_Types...>,
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:419:34: note: member is declared here
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:419:34: note: member is declared here
      _Variadic_union<_Types...> _M_u;
                                 ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:266:38: error: '_M_u' is a private member of 'std::__detail::__variant::_Variant_storage<false, llvm::Record *, unsigned int, std::basic_string<char>>'
                              std::forward<_Variant>(__v)._M_u);
                                                          ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:951:24: note: in instantiation of function template specialization 'std::__detail::__variant::__get<0UL, std::variant<llvm::Record *, unsigned int, std::basic_string<char>>>' requested here
            return __variant::__get<__index>(std::forward<_Variant>(__var));
                              ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:973:8: note: in instantiation of function template specialization 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:476:18) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&>, std::integer_sequence<unsigned long, 0>>::__element_by_index_or_cookie<0UL, std::variant<llvm::Record *, unsigned int, std::basic_string<char>>>' requested here
              __element_by_index_or_cookie<__indices>(
              ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:980:9: note: in instantiation of member function 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:476:18) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&>, std::integer_sequence<unsigned long, 0>>::__visit_invoke_impl' requested here
        return __visit_invoke_impl(std::forward<_Visitor>(__visitor),
               ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:996:11: note: in instantiation of member function 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:476:18) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&>, std::integer_sequence<unsigned long, 0>>::__do_visit_invoke' requested here
          return __do_visit_invoke(std::forward<_Visitor>(__visitor),
                 ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:1005:29: note: in instantiation of member function 'std::__detail::__variant::__gen_vtable_impl<true, std::__detail::__variant::_Multi_array<std::__detail::__variant::__variant_cookie (*)((lambda at /rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:476:18) &&, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&)>, std::tuple<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> &&>, std::integer_sequence<unsigned long, 0>>::__visit_invoke' requested here
      { return _Array_type{&__visit_invoke}; }
                            ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:927:48: note: (skipping 12 contexts in backtrace; use -ftemplate-backtrace-limit=0 to see all)
                std::index_sequence<__indices..., __index>>::_S_apply();
                                                             ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/stl_vector.h:453:14: note: in instantiation of function template specialization 'std::__relocate_a<std::variant<llvm::Record *, unsigned int, std::basic_string<char>> *, std::variant<llvm::Record *, unsigned int, std::basic_string<char>> *, std::allocator<std::variant<llvm::Record *, unsigned int, std::basic_string<char>>>>' requested here
        return std::__relocate_a(__first, __last, __result, __alloc);
                    ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/stl_vector.h:466:9: note: in instantiation of member function 'std::vector<std::variant<llvm::Record *, unsigned int, std::basic_string<char>>>::_S_do_relocate' requested here
        return _S_do_relocate(__first, __last, __result, __alloc, __do_it{});
               ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/vector.tcc:461:23: note: in instantiation of member function 'std::vector<std::variant<llvm::Record *, unsigned int, std::basic_string<char>>>::_S_relocate' requested here
              __new_finish = _S_relocate(__old_start, __position.base(),
                             ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/bits/vector.tcc:121:4: note: in instantiation of function template specialization 'std::vector<std::variant<llvm::Record *, unsigned int, std::basic_string<char>>>::_M_realloc_insert<llvm::Record *>' requested here
          ^
          ^
/checkout/src/llvm-project/llvm/utils/TableGen/DFAEmitter.cpp:324:15: note: in instantiation of function template specialization 'std::vector<std::variant<llvm::Record *, unsigned int, std::basic_string<char>>>::emplace_back<llvm::Record *>' requested here
      Actions.emplace_back(R->getValueAsDef(A));
              ^
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:1220:7: note: constrained by private inheritance here
    : private __detail::__variant::_Variant_base<_Types...>,
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:419:34: note: member is declared here
/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/variant:419:34: note: member is declared here
      _Variadic_union<_Types...> _M_u;
10 errors generated.
10 errors generated.
gmake[2]: *** [utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/DFAEmitter.cpp.o] Error 1
gmake[2]: *** Waiting for unfinished jobs....
[  2%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/APInt.cpp.o
[  2%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/APSInt.cpp.o
[  2%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ARMBuildAttrs.cpp.o
[  2%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ARMAttributeParser.cpp.o
---
[  3%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ErrorHandling.cpp.o
[  3%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ExtensibleRTTI.cpp.o
[  3%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/FileCollector.cpp.o
[  3%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/FileUtilities.cpp.o
gmake[1]: *** [utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/all] Error 2
gmake[1]: *** Waiting for unfinished jobs....
[  3%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/FoldingSet.cpp.o
[  4%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/FormattedStream.cpp.o
[  4%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/FormatVariadic.cpp.o
[  4%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/GlobPattern.cpp.o
---
gmake: *** [all] Error 2
thread 'main' panicked at '
command did not execute successfully, got: exit status: 2

build script failed, must exit now', /cargo/registry/src/index.crates.io-6f17d22bba15001f/cmake-0.1.48/src/lib.rs:975:5
##[endgroup]
 finished in 37.746 seconds
Build completed unsuccessfully in 0:01:17
stage-build INFO: Section `Stage 1 (LLVM PGO) > Build rustc and LLVM` ended: FAIL (77.08s)
---
Total duration:                           1m 17s
------------------------------------------------
root INFO: Free disk space: 517.22 GiB out of total 581.32 GiB (11.02% used)
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 870, in <module>
    run(runner)
  File "../src/ci/stage-build.py", line 861, in run
    raise e
  File "../src/ci/stage-build.py", line 858, in run
    execute_build_pipeline(timer, pipeline, runner, build_args)
  File "../src/ci/stage-build.py", line 781, in execute_build_pipeline
    LLVM_PROFILE_DIR=str(pipeline.llvm_profile_dir_root() / "prof-%p")
  File "../src/ci/stage-build.py", line 608, in build_rustc
    cmd(arguments, env=env)
  File "../src/ci/stage-build.py", line 451, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "/usr/lib64/python3.6/subprocess.py", line 438, in run
    output=stdout, stderr=stderr)
subprocess.CalledProcessError: Command '['/usr/bin/python3', '/checkout/x.py', 'build', '--target', 'x86_64-unknown-linux-gnu', '--host', 'x86_64-unknown-linux-gnu', '--stage', '2', 'library/std', '--llvm-profile-generate']' returned non-zero exit status 1.
