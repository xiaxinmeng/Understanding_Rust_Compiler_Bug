rust

        // Finally, let's make sure it actually appears to remain valid code
        let proc_res = self.typecheck_source(actual);
        if !proc_res.status.success() {
            self.fatal_proc_rec("pretty-printed source does not typecheck", &proc_res);
        }
