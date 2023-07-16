c++

struct SV {
        size_t capacity;
        size_t disc;
        size_t data[40];

        static SV make() {
                SV ret;
                ret.capacity = 0;
                ret.disc = 0;
                return ret;
        }
};

struct L {
        SV a;
};

template<class T>
struct Allocation {
    T *vec;
    void init(T s) {
        *vec = s;
    }
};

void bar(Allocation<L> a, double g) {
        L s = { SV::make() };
        a.init(s);
}

