
% git grep '#!/.*sh'
configure:1:#!/bin/sh
src/ci/docker/disabled/dist-x86_64-dragonfly/build-toolchain.sh:1:#!/usr/bin/env bash
src/ci/docker/disabled/dist-x86_64-haiku/build-toolchain.sh:1:#!/usr/bin/env bash
src/ci/docker/disabled/dist-x86_64-haiku/fetch-packages.sh:1:#!/usr/bin/env bash
src/ci/docker/disabled/dist-x86_64-haiku/llvm-config.sh:1:#!/bin/sh
src/ci/docker/disabled/wasm32-exp/node.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-aarch64-linux/build-toolchains.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-arm-linux/build-toolchains.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-armhf-linux/build-toolchains.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-armv7-linux/build-toolchains.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-powerpc-linux/build-powerpc-toolchain.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-powerpc64-linux/build-powerpc64-toolchain.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-powerpc64le-linux/build-powerpc64le-toolchain.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-s390x-linux/build-s390x-toolchain.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-various-1/build-rumprun.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-various-1/install-x86_64-redox.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-various-2/build-cloudabi-toolchain.sh:1:#!/bin/bash
src/ci/docker/dist-various-2/build-fuchsia-toolchain.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-various-2/build-fuchsia-toolchain.sh:59:#!/bin/sh
src/ci/docker/dist-various-2/build-solaris-toolchain.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-x86_64-linux/build-binutils.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-x86_64-linux/build-clang.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-x86_64-linux/build-cmake.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-x86_64-linux/build-curl.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-x86_64-linux/build-gcc.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-x86_64-linux/build-git.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-x86_64-linux/build-headers.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-x86_64-linux/build-openssl.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-x86_64-linux/build-python.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-x86_64-netbsd/build-netbsd-toolchain.sh:1:#!/usr/bin/env bash
src/ci/docker/dist-x86_64-netbsd/build-netbsd-toolchain.sh:67:#!/usr/bin/env bash
src/ci/docker/dist-x86_64-netbsd/build-netbsd-toolchain.sh:72:#!/usr/bin/env bash
src/ci/docker/run.sh:1:#!/usr/bin/env bash
src/ci/docker/scripts/android-start-emulator.sh:1:#!/bin/sh
src/ci/docker/scripts/freebsd-toolchain.sh:1:#!/bin/bash
src/ci/docker/scripts/freebsd-toolchain.sh:99:#!/bin/sh
src/ci/docker/scripts/qemu-bare-bones-rcS:1:#!/bin/sh
src/ci/docker/x86_64-gnu-tools/checktools.sh:1:#!/bin/sh
src/ci/docker/x86_64-gnu-tools/repo.sh:1:#!/bin/sh
src/ci/init_repo.sh:1:#!/usr/bin/env bash
src/ci/run.sh:1:#!/usr/bin/env bash
src/etc/cat-and-grep.sh:1:#!/bin/sh
src/etc/installer/pkg/postinstall:1:#!/bin/sh
src/etc/rust-gdb:1:#!/bin/sh
src/etc/rust-gdbgui:1:#!/bin/sh
src/etc/rust-lldb:1:#!/bin/sh
src/test/run-make/git_clone_sha1.sh:1:#!/bin/bash -x
src/tools/build-manifest/README.md:13:#!/bin/bash
% git grep '#!/.*sh' | wc -l
      48
%
