
let (chan, port) = pipes::oneshot();
task::spawn |move chan| {
    pipes::send_one(chan, true);
    //pipes::send_one(move chan, true);// Neither work
}
