shell
>    # Update package mirrors (may be needed if you have a fresh install of MSYS2)
>    pacman -Sy pacman-mirrors
> 
>    # Install build tools needed for Rust. If you're building a 32-bit compiler,
>    # then replace "x86_64" below with "i686". If you've already got git, python,
>    # or CMake installed and in PATH you can remove them from this list. Note
>    # that it is important that you do **not** use the 'python2', 'cmake' and 'ninja'
>    # packages from the 'msys2' subsystem. The build has historically been known
>    # to fail with these packages.
>    pacman -S git \
>                make \
>                diffutils \
>                tar \
>                mingw-w64-x86_64-python \
>                mingw-w64-x86_64-cmake \
>                mingw-w64-x86_64-gcc \
>                mingw-w64-x86_64-ninja
> 