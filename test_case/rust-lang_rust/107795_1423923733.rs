
   unsafe {
        let maybe_status = syscall!(Sysno::arch_prctl, ARCH_REQ_XCOMP_PERM, XFEATURE_XTILEDATA);
        match maybe_status {
            Ok(s) => status = s,
            Err(err) => {
                println!("AMX Error: XFEATURE_XTILEDATA setup is failed, TMUL usage is not allowed! Error: {}",err);
                return false;          //<========================================== PROGRAM EXITS HERE!!!
            }
        }
    }
