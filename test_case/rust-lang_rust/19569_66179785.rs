
fn test_insert() {
    // This test checks that every single combination of tail position, length, and
    // insertion position is tested. Capacity 7 should be large enough to cover every case.

    let mut tester = RingBuf::with_capacity(7);
    // can't guarantee we got 7, so have to get what we got.
    // 7 would be great, but we will definitely get 2^k - 1, for k >= 3, or else
    // this test isn't covering what it wants to
    let cap = tester.capacity(); 


    // len is the length *after* insertion
    for len in range(1, cap + 1) {
        // 0, 1, 2, .., len - 1
        let expected: RingBuf<i32> = iter::count(0, 1).take(len).collect();
        for tail_pos in range(0, cap) {
            for to_insert in range(0, len) {
                tester.len = 0;
                tester.tail = tester.head = tail_pos;
                for i in range(0, len) {
                    if i != to_insert {
                        tester.push_back(i);
                    }
                }
                tester.insert(to_insert, to_insert);
                assert_eq!(tester, expected);
            }
        }
    }
}
