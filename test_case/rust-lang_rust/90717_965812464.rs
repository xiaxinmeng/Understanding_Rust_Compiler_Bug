rust
        if self.sess.target.is_like_osx {
            if !self.is_ld {
                arg.push("-Wl,")
            }
            arg.push("-exported_symbols_list,");
        } else if self.sess.target.is_like_solaris {
            if !self.is_ld {
                arg.push("-Wl,")
            }
            arg.push("-M,");
        } else {
            if !self.is_ld {
                arg.push("-Wl,")
            }
            // Both LD and LLD accept export list in *.def file form, there are no flags required
            if !is_windows {
                arg.push("--version-script=")
            }
        }

        arg.push(&path);
        self.cmd.arg(arg);
