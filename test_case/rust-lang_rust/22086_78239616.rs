
for f in $(ls -tr rust-placer/objdir-opt/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/stamp.* ) ; do \
   F=$(basename $f | sed -e s/^stamp./lib/ ) ; \
  ( echo $F; \
    ( ls -l rust-plbase/objdir-opt/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/$F-*.rlib && \
     ls -l rust-placer/objdir-opt/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/$F-*.rlib )  | \
  cut -f 8 -d ' ' ) | xargs  ; done 
