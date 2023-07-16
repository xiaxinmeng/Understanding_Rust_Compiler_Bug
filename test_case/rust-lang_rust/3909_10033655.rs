
struct TestNode {
    data: @TestData
}

struct TestData {
    parent: Option<TestNode>
}

fn main() {
    let node = TestNode {
        data: @TestData {
            parent: None
        }
    };
}
