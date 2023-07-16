
923        if !ignore {
924            ignore |= ignore_llvm(config, ln);
 ......
948            ignore |= !has_rust_lld && config.parse_name_directive(ln, "needs-rust-lld");
949        }
