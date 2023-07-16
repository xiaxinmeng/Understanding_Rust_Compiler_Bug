
void @foo() {
entry:
  %obj = alloca i32
  invoke void @bar() to label %normal unwind label %unwind
unwind:
  %r = landingpad ... cleanup
  call void @drop_obj(i32 %obj)
...
