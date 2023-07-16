c++
#include <iomanip>
#include <iostream>
#include <cassert>

using namespace std;

int main() {
    double a = -25252734735360000.00 * 365.00 + (-25252734735360000.00 * 0.2425);
    double b = -25252734735360000.00 * 365.2425;

    cout << setprecision(32);
    cout << "a = " << a << endl;
    cout << "b = " << b << endl;

    assert(a == b);

    return 0;
}
