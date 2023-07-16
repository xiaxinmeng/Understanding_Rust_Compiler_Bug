rust
                for ril in ipb.wrap_iter(fh.lines()) {
                    if sigdat.got_ctrlc() {
                        break;
                    }
                    let ril = ril.trim_start();

                    /* ... */

                    ips.set_message(ril.to_string());
                    if iwq.send(file).is_err() {
                        break;
                    }
                }
