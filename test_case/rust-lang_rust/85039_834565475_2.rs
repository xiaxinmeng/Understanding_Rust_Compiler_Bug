
RelsExt::from_path(&latest_version.path())
          .expect(&format!("Failed to parse RELS-EXT: {}", latest_version.path().to_string_lossy()))
