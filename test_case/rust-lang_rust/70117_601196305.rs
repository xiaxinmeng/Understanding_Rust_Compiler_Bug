console
$ sudo apt-get install musl-tools
$ git clone --single-branch -b rust_segfault https://github.com/haraldh/enarx_sev_kvm_demo.git
$ cd enarx_sev_kvm_demo/kernel/
$ while cargo build -vvv --release ; do cargo clean; done; 
