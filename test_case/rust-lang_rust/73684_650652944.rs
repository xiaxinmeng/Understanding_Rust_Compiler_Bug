rust
            }
            // FIXME(#73156): Handle source code coverage in const eval
            sym::count_code_region => (),
            _ => return Ok(false),
