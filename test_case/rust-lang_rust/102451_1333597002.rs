rust
use std::cmp::Ordering;
use std::cell::Cell;
use std::sync::atomic::{self, AtomicUsize};

static NUM_SOLIDS: AtomicUsize = AtomicUsize::new(0);

// Gas is always greater than Solid.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
enum Value {
    Solid(usize),
    Gas,
}

// When comparing two EvilValue's it will always give a consistent answer, but
// for some 'mysterious' reason new elements always appear to be greater than
// previously seen ones. This is done by starting all values as Gas and only
// solidifying when necessary: Gas <-> Gas comparisons. Solid <-> Gas
// comparisons can always return that the Gas is greater without any commitment
// to any particular value being made.
#[derive(Clone, Debug)]
struct EvilValue {
    value: Cell<Value>,
}

impl EvilValue {
    pub fn new() -> Self {
        Self { value: Cell::new(Value::Gas) }
    }

    // Note that this doesn't violate any previous returned ordering,
    // as we create a new solid that is larger than any existing solids
    // which is consistent with any earlier Solid <-> Gas comparison we've made.
    pub fn solidify(&self) -> usize {
        if let Value::Solid(id) = self.value.get() {
            id
        } else {
            let id = NUM_SOLIDS.fetch_add(1, atomic::Ordering::Relaxed);
            assert!(id < usize::MAX);
            self.value.set(Value::Solid(id));
            id
        }
    }
}

impl PartialOrd for EvilValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.value.get() == Value::Gas { other.solidify(); }
        self.value.partial_cmp(&other.value)
    }
}

impl PartialEq for EvilValue {
    fn eq(&self, other: &Self) -> bool {
        if self.value.get() == Value::Gas { other.solidify(); }
        self.value == other.value
    }
}

impl Eq for EvilValue { }
impl Ord for EvilValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    for n in [10, 100, 1000, 10000, 100_000] {
        // Construct worst case.
        let mut values: Vec<_> = (0..n).map(|i| (EvilValue::new(), i)).collect();
        values.select_nth_unstable(n/2);
        let mut worst_case: Vec<usize> = vec![0; n];
        for (ev, i) in values {
            worst_case[i] = ev.solidify();
        }
        
        // Benchmark (ordinary usize comparisons - NOT EvilValue).
        let mut comparisons = 0;
        worst_case.select_nth_unstable_by(n/2, |a, b| {
            comparisons += 1;
            a.cmp(&b)
        });
        println!("{n}: {comparisons}");
    }
}
