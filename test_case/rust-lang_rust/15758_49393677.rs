
waiter 1                waiter 2                signaller
if count <= 0 { 
    enqueue self;
}
                                                count++;
                                                signal waiter 1;
                        if count <= 0 { 
                            // count == 1
                        }   
                        // have the lock
// wakes up
// have the lock
