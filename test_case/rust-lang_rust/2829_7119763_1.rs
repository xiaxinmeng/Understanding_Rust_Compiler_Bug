
let t = task::task().notify_port(p);
for iter::repeat(100) { t.spawn(body) }
