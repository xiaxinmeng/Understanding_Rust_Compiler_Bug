rust
use std::collections::VecDeque;

fn main() {
    let mut tester: VecDeque<usize> = VecDeque::with_capacity(3);

    let cap = tester.capacity();
    for len in 0..=cap {
        for tail in 0..=cap {
            for drain_start in 0..=len {
                for drain_end in drain_start..=len {
                    tester.tail = tail;
                    tester.head = tail;
                    for i in 0..len {
                        tester.push_back(i);
                    }

                    // Check that we drain the correct values
                    println!("Testing {} {} {} {}", len, tail, drain_start, drain_end);
                    let drained: VecDeque<_> = tester.drain(drain_start..drain_end).collect();
                    let drained_expected: VecDeque<_> = (drain_start..drain_end).collect();
                    assert_eq!(drained, drained_expected);

                    // We shouldn't have changed the capacity or made the
                    // head or tail out of bounds
                    assert_eq!(tester.capacity(), cap);
                    assert!(tester.tail < tester.cap());
                    assert!(tester.head < tester.cap());

                    // We should see the correct values in the VecDeque
                    let expected: VecDeque<_> = (0..drain_start)
                        .chain(drain_end..len)
                        .collect();
                    assert_eq!(expected, tester);
                }
            }
        }
    }
}
