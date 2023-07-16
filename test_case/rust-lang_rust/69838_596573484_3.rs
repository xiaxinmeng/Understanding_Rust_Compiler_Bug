rust
        log::debug!("parse_external_mod, mp(ownership={:?}, path={:?})", mp.ownership, mp.path);

        // Actually parse the external file as amodule.
        let mut p0 = new_sub_parser_from_file(sess, &mp.path, Some(id.to_string()), span);
