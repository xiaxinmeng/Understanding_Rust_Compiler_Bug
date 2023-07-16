 rust
describe! vec {
    describe! push {
        before_each {
            let mut v = vec![1u, 2, 3];
        }

        it "should increase the length by 1" {
            let old = v.len();
            v.push(4u);
            // Fluid chain with informative error messages.
            (enforce!(old + 1)).is().equal(v.len());
        }
