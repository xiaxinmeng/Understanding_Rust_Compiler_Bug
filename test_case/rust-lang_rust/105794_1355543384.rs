
; cat Dockerfile 
FROM alpine:latest
ENV SCRIPT RUSTDOCFLAGS="--document-private-items --document-hidden-items" env | grep RUSTDOC
RUN sh -c "$SCRIPT"
; docker build .
Sending build context to Docker daemon  79.49MB
Step 1/3 : FROM alpine:latest
 ---> 49176f190c7e
Step 2/3 : ENV SCRIPT RUSTDOCFLAGS="--document-private-items --document-hidden-items" env | grep RUSTDOC
 ---> Using cache
 ---> cb827fc459ad
Step 3/3 : RUN sh -c "$SCRIPT"
 ---> Running in 94860e857d3b
sh: --document-hidden-items: not found
The command '/bin/sh -c sh -c "$SCRIPT"' returned a non-zero code: 1
