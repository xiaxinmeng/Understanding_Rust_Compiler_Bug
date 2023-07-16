
df0466d0bb807a7266cc8ac9931cd43b3e84b62e is the first bad commit
commit df0466d0bb807a7266cc8ac9931cd43b3e84b62e
Author: Josh Stone <jistone@redhat.com>
Date:   Wed Jan 16 09:59:03 2019 -0800

    Rebase to the llvm-project monorepo
    
    The new git submodule src/llvm-project is a monorepo replacing src/llvm
    and src/tools/{clang,lld,lldb}.  This also serves as a rebase for these
    projects to the new 8.x branch from trunk.
    
    The src/llvm-emscripten fork is unchanged for now.

:100644 100644 c4763612dbf3cc85d8682775b79d3297e6ec8c69 4e368c3ebafd8ba3b766ffd4d21d2339ec4f1852 M      .gitmodules
:100644 100644 6596c5a3d9aff0e5f450ba05c826cb34bcabf3b4 dc9abf84b8e5a4d3b6ab5472883f0997fa0454cc M      COPYRIGHT
:040000 040000 d28fae50cd16a7552c46d90c861785b8f9e24f63 1d444fc6305f36cd4e9a429b32387a5737ec7805 M      src
