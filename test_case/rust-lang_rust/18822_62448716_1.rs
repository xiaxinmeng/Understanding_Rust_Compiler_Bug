 GDB
$ LD_LIBRARY_PATH=/gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib:$LD_LIBRARY_PATH gdb --args /contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/bin/rustdoc --cfg dox --cfg stage2 /contrib/projects/rust/rust/src/librustc/lib.rs
...
#0  0x00007ffff73633f0 in rust_panic ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustrt-4e7c5e5c.so
#1  0x00007ffff7363ab6 in unwind::begin_unwind_inner::h86041e0f95d75c4dE9c ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustrt-4e7c5e5c.so
#2  0x00007ffff2e6ee29 in unwind::begin_unwind::h15144528825213807501 ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/libsyntax-4e7c5e5c.so
#3  0x00007ffff2e43187 in diagnostic::SpanHandler::span_fatal::h6faeeffc14c809dfzUF ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/libsyntax-4e7c5e5c.so
#4  0x00007ffff2e87b9b in parse::lexer::StringReader$LT$$x27a$GT$::fatal_span::h55bd0652042d52b7UUJ ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/libsyntax-4e7c5e5c.so
#5  0x00007ffff2e89e1c in parse::lexer::StringReader$LT$$x27a$GT$::fatal_span_char::h2e2befc6346c6898sWJ ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/libsyntax-4e7c5e5c.so
#6  0x00007ffff2e85545 in parse::lexer::StringReader$LT$$x27a$GT$::advance_token::h7180c916e6f0de2a6YJ
    ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/libsyntax-4e7c5e5c.so
#7  0x00007ffff2e81b85 in parse::lexer::StringReader$LT$$x27a$GT$.Reader::next_token::hba722744aa307ad5yOJ ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/libsyntax-4e7c5e5c.so
#8  0x00007ffff78ef2ec in html::highlight::highlight::h439fcb4bfb15efc1x6i ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustdoc-4e7c5e5c.so
#9  0x00007ffff7903ac8 in html::markdown::render::block::h8d36905b36a23b7cIZl ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustdoc-4e7c5e5c.so
#10 0x00007ffff7a1fdb2 in parse_block.part.18 ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustdoc-4e7c5e5c.so
#11 0x00007ffff7a22e77 in hoedown_document_render ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustdoc-4e7c5e5c.so
#12 0x00007ffff790201d in html::markdown::render::hbffb91308ecf1d0doZl ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustdoc-4e7c5e5c.so
#13 0x00007ffff790acf1 in html::markdown::Markdown$LT$$x27a$GT$.fmt..Show::fmt::h30c1a6206e162523bym
    ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustdoc-4e7c5e5c.so
#14 0x00007ffff73b3512 in fmt::write::h4c40aa9c07b41a24hHy ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustrt-4e7c5e5c.so
#15 0x00007ffff73a85c5 in fmt::Formatter$LT$$x27a$GT$::write_fmt::hd44200941aedb3ddaWy ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustrt-4e7c5e5c.so
#16 0x00007ffff794000b in html::render::document::hc8d99d037786b41dhLo ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustdoc-4e7c5e5c.so
#17 0x00007ffff793b73c in html::render::Item$LT$$x27a$GT$.fmt..Show::fmt::h6df1474bfc3e0461boo ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustdoc-4e7c5e5c.so
#18 0x00007ffff73b348f in fmt::write::h4c40aa9c07b41a24hHy ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustrt-4e7c5e5c.so
#19 0x00007ffff7927e68 in io::Writer::write_fmt::h2764658639759448054 ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustdoc-4e7c5e5c.so
#20 0x00007ffff7939fce in html::render::Context::item::render::h33470e8ebb2d63e9C5n ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustdoc-4e7c5e5c.so
#21 0x00007ffff793793a in html::render::Context::item::closure.26211 ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustdoc-4e7c5e5c.so
#22 0x00007ffff7934b0d in html::render::Context::recurse::h1197179027284130113 ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustdoc-4e7c5e5c.so
#23 0x00007ffff7913eb0 in html::render::run::hfdc7187ac9c09a7c9Fm ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustdoc-4e7c5e5c.so
#24 0x00007ffff797f837 in main_args::h0a752a5313b01ae15Ct ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustdoc-4e7c5e5c.so
#25 0x00007ffff797b86a in main::h8dadf866764c934fRyt ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustdoc-4e7c5e5c.so
#26 0x00007ffff766231d in start::closure.2818 ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/libnative-4e7c5e5c.so
#27 0x00007ffff73b8cfc in rust_try_inner ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustrt-4e7c5e5c.so
#28 0x00007ffff73b8ce6 in rust_try ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustrt-4e7c5e5c.so
#29 0x00007ffff7361313 in unwind::try::hd3616a2e4fe236admYc ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustrt-4e7c5e5c.so
#30 0x00007ffff73611dc in task::Task::run::h35d5a8207977fa8du4b ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/librustrt-4e7c5e5c.so
#31 0x00007ffff766213f in start::hbeb6d12d99d267112ma ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/libnative-4e7c5e5c.so
#32 0x00007ffff7661f76 in lang_start::h48ecbf3d8d8483a4lma ()
   from /gpfs/main/sys/shared/psfu/contrib/projects/rust/rust/x86_64-unknown-linux-gnu/stage2/lib/libnative-4e7c5e5c.so
#33 0x00007ffff6f87ead in __libc_start_main (main=<optimized out>, argc=<optimized out>, 
    ubp_av=<optimized out>, init=<optimized out>, fini=<optimized out>, rtld_fini=<optimized out>, 
    stack_end=0x7fffffffe468) at libc-start.c:244
