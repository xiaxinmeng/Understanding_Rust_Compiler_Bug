plain
Step 7/9 : ENV NO_DOWNLOAD_CI_LLVM 1
 ---> Running in cff9229c632b
Removing intermediate container cff9229c632b
 ---> 16fa0f6b9283
Step 8/9 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-13       --enable-llvm-link-shared       --set rust.randomize-layout=true       --set rust.thin-lto-import-instr-limit=10
Removing intermediate container 8ef778b74a02
 ---> 0ca82e63015e
Step 9/9 : ENV SCRIPT ../x.py --stage 2 test --exclude src/tools/tidy &&            ../x --stage 2 test src/test/mir-opt                              --host='' --target=i686-unknown-linux-gnu &&            ../x.ps1 --stage 2 test src/test/ui --pass=check                              --host='' --target=i686-unknown-linux-gnu &&            python2.7 ../x.py --stage 2 test src/tools/tidy
 ---> Running in c5d9ca78d7c7
 ---> Running in c5d9ca78d7c7
Removing intermediate container c5d9ca78d7c7
 ---> 362ac61ff92b
Successfully built 362ac61ff92b
Successfully tagged rust-ci:latest
Built container sha256:362ac61ff92bd913c29e6dade185a983fd18db198e77f5862ff51c68ce259a69
Uploading finished image to https://ci-caches.rust-lang.org/docker/7321bc21f534cedb88ae0ee86dfaeab6c2f8459a66c6b6c8071093a519d76afaaa33cd8244125b598b4894ade4703600a38dde327f758c861ce610336d9a166a
upload failed: - to s3://rust-lang-ci-sccache2/docker/7321bc21f534cedb88ae0ee86dfaeab6c2f8459a66c6b6c8071093a519d76afaaa33cd8244125b598b4894ade4703600a38dde327f758c861ce610336d9a166a Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
configure: 
configure: build.build          := x86_64-unknown-linux-gnu
configure: target.x86_64-unknown-linux-gnu.llvm-config := /usr/lib/llvm-13/bin/ll ...
configure: llvm.link-shared     := True
configure: rust.randomize-layout := True
configure: rust.codegen-units-std := 1
configure: rust.verify-llvm-ir  := True
configure: llvm.ccache          := sccache
configure: build.submodules     := False
---
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 353 tests
ii.............i.....i.ii.................iii........ii.i.......i.................ii.... 88/353
.............i............i..i.................i...iii........i..i..F...iii.i........i.. 176/353
..i.i.ii.i......i.......iii.......i...i.....................iiiiiiii..i................. 352/353
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.
failures:
failures:

---- [codegen] src/test/codegen/issue-37945.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll" "/checkout/src/test/codegen/issue-37945.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/issue-37945.rs:31:16: error: CHECK-NEXT: expected string not found in input
// CHECK-NEXT: [[C:%.*]] = icmp ne {{i32\*|ptr}} %xs.1, null
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll:17:7: note: scanning from here
      ^
      ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll:18:2: note: possible intended match here
 %0 = icmp ne i32* %xs.0, null

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll
Check file: /checkout/src/test/codegen/issue-37945.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
           .
           .
           .
           .
          12:  ret i1 %_12.i 
          13: } 
          14:  
          15: ; Function Attrs: mustprogress nofree nosync nounwind nonlazybind uwtable willreturn 
          16: define noundef zeroext i1 @is_empty_2(i32* %xs.0, i32* noundef nonnull %xs.1) unnamed_addr #0 { 
          17: start: 
next:31'0           X error: no match found
          18:  %0 = icmp ne i32* %xs.0, null 
next:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
next:31'1      ?                              possible intended match
          19:  tail call void @llvm.assume(i1 %0) 
next:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          20:  %_12.i = icmp eq i32* %xs.0, %xs.1 
next:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          21:  ret i1 %_12.i 
next:31'0     ~~~~~~~~~~~~~~~
          22: } 
next:31'0     ~~
          23:  
next:31'0     ~
           .
           .
>>>>>>
------------------------------------------
