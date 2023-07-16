c++
#include <stdint.h>
#include <cstring>

struct LayoutPrimitiveInfo2 {
        LayoutPrimitiveInfo2() {
                memset(rect, 0, sizeof(rect));
                is_backface_visible = 0;
        }
        float rect[7];
        uint16_t is_backface_visible;
};

struct SpecificDisplayItem2 {
    static SpecificDisplayItem2 PopStackingContext() {
            SpecificDisplayItem2 ret;
            ret.p.a.disc = 0;
            return ret;
    }
    union {
            struct A {
                    uint64_t disc;
            } a;
            struct B {
                    uint64_t disc;
                    double f[22];
            } b;
    } p;
};

struct Outer {
    SpecificDisplayItem2 item;
    LayoutPrimitiveInfo2 info;
};

extern int do_item(Outer &o);

void do_ants() {
    Outer o =  {
                SpecificDisplayItem2::PopStackingContext(),
                LayoutPrimitiveInfo2(),
            };
    do_item(o);
}

