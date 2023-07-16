
11:22 <&brson> posix4e: there are a few steps to making source easily installable: package and deploy it in rust-installer format, teach the channel metadata about it, teach rustup.sh how to install source 
               packages
11:22 <&brson> the first is easy to tackle locally. the second requires hacking our infrastructure
11:22 <&brson> to make source packages you've basically got to add to the makefiles rules like https://github.com/rust-lang/rust/blob/master/mk/dist.mk#L143
11:23 <&brson> it'll take the entire source, probably as *already* tarred here https://github.com/rust-lang/rust/blob/master/mk/dist.mk#L36
11:23 <&brson> then invoke rust-installer to turn it into a rustc-source package
11:24 <&brson> (though maybe std-source would be better? hard to pull out just the std source)
11:24 <&brson> once the makefiles know how to create rustc-source they'll automatically start uploading with nightlies
11:25 <&brson> one wrinkle here is that source is target-agnostic and all our existing bins are target-specific, so named e.g. rustc-*-x86_64-etc
11:25 <&brson> we *might* just go ahead and stick with that scheme, producing a bunch of identical source artifacts that are named differently, per target
11:25 <&brson> but that seems wasteful
11:26 <&brson> i think just calling it rustc-source-{version}.tar.gz is probably fine, but we've got to consider the naming of the existing source tarballs (which is not a rust-installer package), and whether it 
               conflicts
