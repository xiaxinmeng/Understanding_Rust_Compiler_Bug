
let result = do task::try {
    do unkillable { do rekillable {
        let (port,chan) = comm::stream();
        do task::spawn { chan.send(()); fail!(); }
        port.recv(); // wait for child to exist
        port.recv(); // block forever, expect to get killed.
    } }
};
assert!(result.is_err());
