
struct PriorityQueue <T> {
    comparator: fn(&T, &T) -> Ordering
    //other stuff
}

impl <T> PriorityQueue<T> {
    fn with_comparator (comparator: fn(&T, &T) -> Ordering) -> PriorityQueue<T> {
        PriorityQueue{ comparator: comparator, /* other stuff */}
    }
}

fn natural <F:Ord> (a: &F, b: &F) -> Ordering { a.cmp(b) }
fn rev <F:Ord> (a: &F, b: &F) -> Ordering { 
    match a.cmp(b) {
        Equal => Equal,
        Less => Greater,
        Greater => Less,
    } 
}

impl <T: Ord> PriorityQueue<T> {
    fn natural_ordering () -> PriorityQueue<T> {
        PriorityQueue::with_comparator(natural)
    }

    fn rev_ordering () -> PriorityQueue<T> {
        PriorityQueue::with_comparator(rev)
    }

    fn new() -> PriorityQueue<T> {
        PriorityQueue::natural_ordering()
    }
}
