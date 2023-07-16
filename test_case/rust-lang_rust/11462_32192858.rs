
// try_lock() {
//     if atomically {queue_size == 0 && lockers == 0} && lock.try_lock() {
//         if atomically {let b = queue_size == 0; if b {++lockers; true} else {false}} {
//             ok
//         } else {
//             lock.unlock()
//             fail
//         }
//     } else {
//         fail
//     }
// }
// 
// lock() {
//     if try_lock() {
//         return guard;
//     }
//
//     if can_block && atomically {let b = queue_size == 0; if b {++lockers; true} else {false}} {
//         lock.lock();
//     } else {
//         q.push();
//         if atomically {++queue_size; if(lockers == 0) {++lockers; true} else {false}} {
//             // this never blocks indefinitely
//             // this is because lockers was 0, so we have no one having or trying to get the lock
//             // and we atomically set queue_size to a positive value, so no one will start blocking
//             lock.lock();
//             atomically {--queue_size}
//             t = q.pop();
//             if t != ourselves {
//                 t.wakeup();
//                 go to sleep
//             }
//         } else {
//             go to sleep
//         }
//     }
// }
// 
// unlock() {
//     if atomically {if(queue_size != 0 && lockers == 1) {--queue_size; true} else {--lockers; false}} {
//         t = q.pop();
//         t.wakeup();
//     } else {
//         lock.unlock()
//     }
// }
