
---- [pretty] pretty/attr-variant-data.rs stdout ----

	

error: pretty-printed source does not typecheck

status: exit code: 101

command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc - -Zno-trans --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty/attr-variant-data.pretty-out --target=x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty/attr-variant-data.stage2-x86_64-unknown-linux-gnu.pretty.libaux -Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers

stdout:

------------------------------------------

------------------------------------------

stderr:

------------------------------------------

error: cannot find derive macro `Serialize` in this scope

  --> <anon>:46:10

   |

46 | #[derive(Serialize, Deserialize)]

   |          ^^^^^^^^^

error: cannot find derive macro `Serialize` in this scope

  --> <anon>:30:10

   |

30 | #[derive(Serialize, Deserialize)]

   |          ^^^^^^^^^

error: cannot find derive macro `Serialize` in this scope

  --> <anon>:22:10

   |

22 | #[derive(Serialize, Deserialize)]

   |          ^^^^^^^^^

error: cannot find derive macro `Serialize` in this scope

  --> <anon>:19:10

   |

19 | #[derive(Serialize, Deserialize)]

   |          ^^^^^^^^^

error: aborting due to 4 previous errors
