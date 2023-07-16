c++
template<class T>
struct generic {
    T a;
};

struct normal {
    int a;
};

int main() {
    auto generic_t = generic<int> { 10 };
    auto normal_t = normal { 10 };
    return 0;
}
