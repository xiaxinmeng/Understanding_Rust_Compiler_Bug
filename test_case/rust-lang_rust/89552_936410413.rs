
test tests/ui/not_a_struct.rs ... mismatch

EXPECTED:
┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
error: Must derive on a struct

  = help: This macro can only be implemented on a struct.

        #[derive(Hcaptcha)]
        struct MyStruct {
            #[captcha]
            hcaptcha: String,
        }

 --> $DIR/not_a_struct.rs:5:6
  |
5 | enum Test {
  |      ^^^^
┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈

ACTUAL OUTPUT:
┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
error: Must derive on a struct
 --> $DIR/not_a_struct.rs:5:6
  |
5 | enum Test {
  |      ^^^^
  |
  = help: This macro can only be implemented on a struct.

                  #[derive(Hcaptcha)]
                  struct MyStruct {
                      #[captcha]
                      hcaptcha: String,
                  }
┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
note: If the actual output is the correct output you can bless it by rerunning
      your test with the environment variable TRYBUILD=overwrite



test ui ... FAILED
