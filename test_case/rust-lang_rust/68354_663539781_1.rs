rust

                    (Some(previous), status) if *previous == status => {} // no change, ignore
                    (_, status) => {
                        // new status
                        previous_status = Some(status);
                    }
