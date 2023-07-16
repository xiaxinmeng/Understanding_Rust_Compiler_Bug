plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0308]: mismatched types
   --> compiler/rustc_metadata/src/dependency_format.rs:160:44
    |
160 |                 let src: Rc<CrateSource> = tcx.used_crate_source(cnum);
    |                          ---------------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Rc`, found struct `Lrc`
    |                          expected due to this
    |
    = note: expected struct `Rc<CrateSource>`
               found struct `Lrc<CrateSource>`
               found struct `Lrc<CrateSource>`

    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error[E0308]: mismatched types
   --> compiler/rustc_metadata/src/dependency_format.rs:190:36
    |
190 |         let src: Rc<CrateSource> = tcx.used_crate_source(cnum);
    |                  ---------------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Rc`, found struct `Lrc`
    |                  expected due to this
    |
    = note: expected struct `Rc<CrateSource>`
               found struct `Lrc<CrateSource>`
               found struct `Lrc<CrateSource>`

error[E0308]: mismatched types
   --> compiler/rustc_metadata/src/dependency_format.rs:232:36
    |
232 |         let src: Rc<CrateSource> = tcx.used_crate_source(cnum);
    |                  ---------------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Rc`, found struct `Lrc`
    |                  expected due to this
    |
    = note: expected struct `Rc<CrateSource>`
               found struct `Lrc<CrateSource>`
               found struct `Lrc<CrateSource>`

error[E0308]: mismatched types
   --> compiler/rustc_metadata/src/dependency_format.rs:264:36
    |
264 |         let src: Rc<CrateSource> = tcx.used_crate_source(cnum);
    |                  ---------------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Rc`, found struct `Lrc`
    |                  expected due to this
    |
    = note: expected struct `Rc<CrateSource>`
               found struct `Lrc<CrateSource>`
