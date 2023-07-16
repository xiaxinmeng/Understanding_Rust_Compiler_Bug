
commit e66fec8e6ba35edf01f86c2bf6514109aba4ded2
Author: Faisal Abbas <90.abbasfaisal@gmail.com>
Date:   Mon Jun 27 16:05:49 2022 +0100

    gccrs: const folding port
    
    This changeset ports cp_global_trees structure which is used throughout the
    cp constexpr.cc code. I am not sure what it's purpose is but it seems it is used
    to add and manipulate tree information during the c++ compilation process.
    It is possible this is not needed in the Rust code and may be taken out later.
    Also, the initialization function isn't being called from anywhere yet, so
    we will need to find a suitable point for it.
