 c++
class IString {
public:
    int operator==(const IString& other);
private:
    int id;
};
int IString::operator==(const IString& other) {
    return id == other.id;
}

int foo(IString a, IString b) { return a==b; }
