
let (chan, port) = pipes::oneshot();
pipes::send_one(move chan, true);
