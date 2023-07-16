
match output_file {
            TargetLocation::ThisFile(path) => {
                rustc.arg("-o").arg(path);
            }
            TargetLocation::ThisDirectory(path) => {
                if is_rustdoc {
                    // `rustdoc` uses `-o` for the output directory.
                    rustc.arg("-o").arg(path);
                } else {
                    rustc.arg("--out-dir").arg(&self.config.build_base);
                }
            }
        }
 