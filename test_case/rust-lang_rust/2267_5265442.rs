
export ctx::{};

enum ctx { token }
fn gnutls_init() -> ctx {
    let ch = spawn_global_unique { |token_port|
        weaken_task() { |weak_exit_port|
            libgnutls::gnutls_global_init();

            let mut continue = true;
            while continue {
                continue = either::either(
                    { |exit| false },
                    { |token_chan| comm::send(token_chan, token) },
                    comm::select2(weak_exit_port, token_port);
            }

            libgnutls::gnutls_global_deinit();
        }
    };

    comm::recv(ch)
}

fn gnutls_certificate_allocate_credentials(ctx: ctx) { ... }
