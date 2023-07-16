
; docker build .
Sending build context to Docker daemon  79.49MB
Step 1/3 : FROM alpine:latest
 ---> 49176f190c7e
Step 2/3 : ENV SCRIPT RUSTDOCFLAGS=\"--document-private-items --document-hidden-items\" env
 ---> Running in bd1af3f1d278
Removing intermediate container bd1af3f1d278
 ---> efaf4a9fafd7
Step 3/3 : RUN sh -c "$SCRIPT"
 ---> Running in c07667c971ce
RUSTDOCFLAGS=--document-private-items --document-hidden-items
SCRIPT=RUSTDOCFLAGS="--document-private-items --document-hidden-items" env
