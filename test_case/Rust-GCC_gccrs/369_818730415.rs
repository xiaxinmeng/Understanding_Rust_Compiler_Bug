c++
struct AbstractStruct {
    virtual void not_implemented() = 0;
};

template <typename T>
struct Foo {
    T foo() { return T(); }
};

int main() {
    Foo<AbstractStruct>();
}
