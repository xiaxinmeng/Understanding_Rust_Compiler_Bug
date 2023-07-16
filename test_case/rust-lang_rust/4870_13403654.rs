
extern mod std;

struct MyStruct {
    map: std::oldmap::HashMap<int, int>
}

fn f(ms: &MyStruct) {
    ms.map.insert(1, 2);
    ms.map.clear();
}

fn main() {}
