
let chan = SharedChan(move chan);

let a = chan.clone();

do task::spawn |move a| {
    send_actor(a);
}
