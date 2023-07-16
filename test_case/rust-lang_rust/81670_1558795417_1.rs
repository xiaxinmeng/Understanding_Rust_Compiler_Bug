rustc
warning: allow(unreachable_code) incompatible with previous forbid
   --> src/main.rs:109:29
    |
109 |         let target_binary = fs::read(target_file)?;
    |                             ^^^^^^^^^^^^^^^^^^^^^^ overruled by previous forbid
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
    = note: `forbid` lint level was set on command line
    = note: `-W forbidden-lint-groups` implied by `-W future-incompatible`

warning: allow(unreachable_code) incompatible with previous forbid
   --> src/main.rs:125:26
    |
125 |         let elf_object = object::File::parse(&*target_binary)?;
    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overruled by previous forbid
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
    = note: `forbid` lint level was set on command line

warning: allow(unreachable_code) incompatible with previous forbid
   --> src/main.rs:137:13
    |
137 |             disassemble_binary(elf_object, capstone_object)?;
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overruled by previous forbid
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
    = note: `forbid` lint level was set on command line

warning: allow(unreachable_code) incompatible with previous forbid
  --> src/main.rs:66:24
   |
66 |       let section_text = elf_object
   |  ________________________^
67 | |         .section_by_name(".text")
68 | |         .ok_or("unable to get the `.text` section from the provided binary")?;
   | |_____________________________________________________________________________^ overruled by previous forbid
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unreachable_code) incompatible with previous forbid
  --> src/main.rs:70:16
   |
70 |     let code = section_text.data()?;
   |                ^^^^^^^^^^^^^^^^^^^^ overruled by previous forbid
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unreachable_code) incompatible with previous forbid
  --> src/main.rs:79:28
   |
79 |               let mnemonic = instruction
   |  ____________________________^
80 | |                 .mnemonic()
81 | |                 .ok_or("ERROR: unable to get mnemonic")?;
   | |________________________________________________________^ overruled by previous forbid
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unreachable_code) incompatible with previous forbid
  --> src/main.rs:82:28
   |
82 |               let operands = instruction
   |  ____________________________^
83 | |                 .op_str()
84 | |                 .ok_or("ERROR: unable to get operation operands")?;
   | |__________________________________________________________________^ overruled by previous forbid
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: `inverse-transpiler` (bin "inverse-transpiler") generated 7 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
