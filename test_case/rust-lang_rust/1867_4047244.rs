
let po = comm::port();
let ch = comm::chan(po);
let ch = spawn_listener {|po|
    // Now the child has a port called 'po' to read from and
    // an environment-captured channel called 'ch'.
};
// Likewise, the parent has both a 'po' and 'ch'
