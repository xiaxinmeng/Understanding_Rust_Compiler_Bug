
use std::sync::mpmc_bounded_queue;

mod work;

void main() {
    let work_queue = mpmc_bounded_queue::Queue::<work::Work>::with_capacity(8);
}
