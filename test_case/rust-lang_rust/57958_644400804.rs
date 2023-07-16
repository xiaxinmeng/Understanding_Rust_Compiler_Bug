bash
 Using docker image sha256:5ebf5da012391d1556e312fb199d7172c202c7d6b51bac71d423ff7d4a64645e for registry.gitlab.com/veloren/veloren-docker-ci:latest ...

ln -s /dockercache/cache-all target
$ cargo test
   Compiling veloren-common v0.6.0 (/builds/veloren/veloren/common)
   Compiling veloren-voxygen v0.6.0 (/builds/veloren/veloren/voxygen)
   Compiling veloren_network v0.1.0 (/builds/veloren/veloren/network)
   Compiling veloren-world v0.6.0 (/builds/veloren/veloren/world)
   Compiling veloren-client v0.6.0 (/builds/veloren/veloren/client)
   Compiling veloren-server v0.6.0 (/builds/veloren/veloren/server)
   Compiling veloren-chat-cli v0.6.0 (/builds/veloren/veloren/chat-cli)
error: Failed to delete invalidated or incompatible incremental compilation session directory contents `/dockercache/cache-all/debug/incremental/veloren_voxygen-29cc5aon4ib6e/s-foe120g0hi-17gfq9r-working/dep-graph.bin`: No such file or directory (os error 2).
error: aborting due to previous error
error: could not compile `veloren-voxygen`.
To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
