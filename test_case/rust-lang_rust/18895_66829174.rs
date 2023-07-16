 rust
 639     let copy_if_one_unit = |ext: &str, output_type: config::OutputType, keep_numbered: bool| {
 640         // Three cases:
 641         if sess.opts.cg.codegen_units == 1 {
 642             // 1) Only one codegen unit.  In this case it's no difficulty
 643             //    to copy `foo.0.x` to `foo.x`.
 644             fs::copy(&crate_output.with_extension(ext),
 645                      &crate_output.path(output_type)).unwrap();
 646             if !sess.opts.cg.save_temps && !keep_numbered {
 647                 // The user just wants `foo.x`, not `foo.0.x`.
 648                 remove(sess, &crate_output.with_extension(ext));
 649             }
 650         } else {
 651             if crate_output.single_output_file.is_some() {
 652                 // 2) Multiple codegen units, with `-o some_name`.  We have
 653                 //    no good solution for this case, so warn the user.
 654                 sess.warn(format!("ignoring -o because multiple .{} files were produced",
 655                                   ext).as_slice());
 656             } else {
 657                 // 3) Multiple codegen units, but no `-o some_name`.  We
 658                 //    just leave the `foo.0.x` files in place.
 659                 // (We don't have to do any work in this case.)
 660             }
 661         }
 662     };
