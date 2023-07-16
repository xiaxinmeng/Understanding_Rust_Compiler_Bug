rust
let [neg @ ..2, pos @ 2..] = x; // error
assert_eq!(neg, [-100, -50]);
assert_eq!(pos, [50, 100]);

let [_, fifties @ 1..3, _] = x; // error
assert_eq!(fifties, [-50, 50])
