
~/lfs  [ mbenfield ]
% rustc +lfs main.rs -o mainlfs
~/lfs  [ mbenfield ]
% nm mainlfs | rg 'U (open|openat|sendfile|mmap)'
                 U mmap64@GLIBC_2.2.5
                 U open64@GLIBC_2.2.5
                 U openat64@GLIBC_2.4
                 U sendfile64@GLIBC_2.3
~/lfs  [ mbenfield ]
% rustc main.rs -o main
~/lfs  [ mbenfield ]
% nm main | rg 'U (open|openat|sendfile|mmap)'
                 U mmap@GLIBC_2.2.5
                 U open64@GLIBC_2.2.5
                 U openat@GLIBC_2.4
                 U open@GLIBC_2.2.5
                 U sendfile@GLIBC_2.2.5
~/lfs  [ mbenfield ]
% uname -rv
5.18.16-1rodete4-amd64 #1 SMP PREEMPT_DYNAMIC Debian 5.18.16-1rodete4 (2022-10-21)
~/lfs  [ mbenfield ]
% cat main.rs
fn main() {
    ::std::fs::remove_dir_all("/some/fake/directory").unwrap();
    let mut f1 = ::std::fs::File::open("filename1").unwrap();
    let mut f2 = ::std::fs::File::create("filename2").unwrap();
    ::std::io::copy(&mut f2, &mut f1).unwrap();
}
