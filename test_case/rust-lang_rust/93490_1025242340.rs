C++
struct Foo
{
    float bar1;
    float bar2;
    float bar3;
    float bar4;
};

Foo sum_cpp(Foo foo1, Foo foo2)
{
    Foo foo3;
    foo3.bar1 = foo1.bar1 + foo2.bar1;
    foo3.bar2 = foo1.bar2 + foo2.bar2;
    foo3.bar3 = foo1.bar3 + foo2.bar3;
    foo3.bar4 = foo1.bar4 + foo2.bar4;
    return foo3;
}
