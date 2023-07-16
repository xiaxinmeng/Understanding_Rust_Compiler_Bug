
(0, lockers) => (0, lockers + 1) // task locks with nothing queued
(queue_size, lockers > 1) => (queue_size, lockers - 1) // task unlocks with native task waiting
(0, 1) => (0, 0) // task unlocks with nothing else waiting

(queue_size, lockers) => (queue_size + 1, lockers) // task queues up
(queue_size > 0, 1) => (queue_size - 1, lockers) // task unlocks, waking up queued task
