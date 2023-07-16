
template<class T>
auto foo()
{
    if constexpr(compile_time_compute_value_changing_depending_on_T<T>()) {
        return Type1();
    else {
        return Type2();
    }
}
