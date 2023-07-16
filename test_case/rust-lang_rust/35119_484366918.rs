rust
global_asm! {r#"
  .section  __TEXT,__text,regular,pure_instructions
  .globl  _do_stuff_in_asm
_do_stuff_in_asm:
  # ... code that consumes L_SPECIAL_PRIVATE_SYMBOL ...
  retq

  .section	__TEXT,__special_section
L_SPECIAL_PRIVATE_SYMBOL:
	.asciz	"foo"
"#}

pub fn do_stuff_in_rust() -> &'static [u8; 4] {
  extern "C" {
    #[link_name = "\x01L_SPECIAL_PRIVATE_SYMBOL"]
    static foo: &'static [u8; 4];
  }
  return unsafe { foo };
}
