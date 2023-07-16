plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 9cf05f3614ef00c0da82b0594704aff213ab7c34 and 9e3d6c16a5d6a9fa597cb9ab270344d5a14feb3b
Executing the job since rustdoc was updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
  acl adwaita-icon-theme at-spi2-core colord colord-data dbus
  dconf-gsettings-backend dconf-service distro-info-data fontconfig
  fontconfig-config fonts-dejavu-core gconf-service-backend gconf2-common
  glib-networking glib-networking-common glib-networking-services
  gsettings-desktop-schemas hicolor-icon-theme humanity-icon-theme ifupdown
  indicator-application iproute2 isc-dhcp-client isc-dhcp-common
  libappindicator3-1 libasound2-data libatk1.0-data libatm1 libatspi2.0-0
  libboost-filesystem1.58.0 libboost-system1.58.0 libbsd0 libcairo-gobject2
  libcap-ng0 libcapnp-0.5.3 libcolord2 libcolorhug2 libcroco3 libdatrie1
  libdbus-glib-1-2 libdbusmenu-glib4 libdbusmenu-gtk3-4 libdbusmenu-gtk4
  libdconf1 libdns-export162 libdrm-amdgpu1 libdrm-common libdrm-intel1
---
  libtext-iconv-perl libthai-data libthai0 libtie-ixhash-perl libtiff5
  libtimedate-perl libtxc-dxtn-s2tc0 liburi-perl libusb-1.0-0 libvpx3
  libwayland-client0 libwayland-cursor0 libwayland-egl1-mesa
  libwayland-server0 libwww-perl libwww-robotrules-perl libx11-data
  libx11-protocol-perl libxau6 libxaw7 libxcb-dri2-0 libxcb-dri3-0 libxcb-glx0
  libxcb-present0 libxcb-render0 libxcb-shape0 libxcb-shm0 libxcb-sync1
  libxcb-xfixes0 libxdmcp6 libxft2 libxinerama1 libxkbcommon0
  libxml-parser-perl libxml-twig-perl libxml-xpathengine-perl libxmu6 libxmuu1
  libxpm4 libxshmfence1 libxt6 libxtables11 libxv1 libxxf86dga1 libxxf86vm1
  netbase policykit-1 shared-mime-info ubuntu-mono ucf x11-common x11-utils
  x11-xserver-utils xdg-user-dirs xkb-data
Suggested packages:
  colord-sensor-argyll dbus-user-session | dbus-x11 ppp rdnssd iproute2-doc
  resolvconf avahi-autoipd isc-dhcp-client-ddns apparmor libasound2-plugins
  alsa-utils libdigest-hmac-perl libgssapi-perl cups-common libgd-tools
  gphoto2 gvfs libdata-dump-perl liblcms2-utils libcrypt-ssleay-perl pciutils
  librsvg2-bin avahi-daemon hplip libsane-extras sane-utils lm-sensors
  libauthen-ntlm-perl libunicode-map8-perl libunicode-string-perl
  xml-twig-tools lsb mesa-utils nickle cairo-5c xorg-docs-core gvfs-bin
  acl adwaita-icon-theme at-spi2-core colord colord-data dbus
  dconf-gsettings-backend dconf-service distro-info-data fontconfig
  fontconfig-config fonts-dejavu-core fonts-liberation gconf-service
  gconf-service-backend gconf2-common glib-networking glib-networking-common
  gconf-service-backend gconf2-common glib-networking glib-networking-common
  glib-networking-services gsettings-desktop-schemas hicolor-icon-theme
  humanity-icon-theme ifupdown indicator-application iproute2 isc-dhcp-client
  isc-dhcp-common libappindicator1 libappindicator3-1 libasound2
  libasound2-data libatk-bridge2.0-0 libatk1.0-0 libatk1.0-data libatm1
  libatspi2.0-0 libauthen-sasl-perl libavahi-client3 libavahi-common-data
  libcairo-gobject2 libcairo2 libcap-ng0 libcapnp-0.5.3 libcolord2
  libcolorhug2 libcroco3 libcups2 libdatrie1 libdbus-1-3 libdbus-glib-1-2
  libdbusmenu-glib4 libdbusmenu-gtk3-4 libdbusmenu-gtk4 libdconf1
  libdns-export162 libdrm-amdgpu1 libdrm-common libdrm-intel1 libdrm-nouveau2
---
  libwayland-server0 libwww-perl libwww-robotrules-perl libx11-6 libx11-data
  libx11-protocol-perl libx11-xcb1 libxau6 libxaw7 libxcb-dri2-0 libxcb-dri3-0
  libxcb-glx0 libxcb-present0 libxcb-render0 libxcb-shape0 libxcb-shm0
  libxcb-sync1 libxcb-xfixes0 libxcb1 libxcomposite1 libxcursor1 libxdamage1
  libxdmcp6 libxext6 libxfixes3 libxft2 libxi6 libxinerama1 libxkbcommon0
  libxml-parser-perl libxml-twig-perl libxml-xpathengine-perl libxmu6 libxmuu1
  libxpm4 libxrandr2 libxrender1 libxshmfence1 libxss1 libxt6 libxtables11
  libxtst6 libxv1 libxxf86dga1 libxxf86vm1 lsb-release netbase policykit-1
  shared-mime-info ubuntu-mono ucf wget x11-common x11-utils x11-xserver-utils
  xdg-user-dirs xdg-utils xkb-data
Need to get 54.1 MB of archives.
After this operation, 350 MB of additional disk space will be used.
Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libatm1 amd64 1:2.5.1-1.5 [24.2 kB]
Get:2 http://archive.ubuntu.com/ubuntu xenial/main amd64 libmnl0 amd64 1.0.3-5 [12.0 kB]
---
Get:133 http://archive.ubuntu.com/ubuntu xenial/main amd64 libxtst6 amd64 2:1.2.2-1 [14.1 kB]
Get:134 http://archive.ubuntu.com/ubuntu xenial/main amd64 libxv1 amd64 2:1.0.10-1 [10.3 kB]
Get:135 http://archive.ubuntu.com/ubuntu xenial/main amd64 libxxf86dga1 amd64 2:1.1.4-1 [13.7 kB]
Get:136 http://archive.ubuntu.com/ubuntu xenial/main amd64 libxxf86vm1 amd64 1:1.1.4-1 [10.6 kB]
Get:137 http://archive.ubuntu.com/ubuntu xenial/main amd64 libtxc-dxtn-s2tc0 amd64 0~git20131104-1.1 [51.8 kB]
Get:139 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 iproute2 amd64 4.3.0-1ubuntu3.16.04.5 [523 kB]
Get:140 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ifupdown amd64 0.8.10ubuntu1.4 [54.9 kB]
Get:141 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libisc-export160 amd64 1:9.10.3.dfsg.P4-8ubuntu1.19 [153 kB]
Get:142 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libdns-export162 amd64 1:9.10.3.dfsg.P4-8ubuntu1.19 [665 kB]
---
Preparing to unpack .../hicolor-icon-theme_0.15-0ubuntu1.1_all.deb ...
Unpacking hicolor-icon-theme (0.15-0ubuntu1.1) ...
Selecting previously unselected package libgtk-3-bin.
Preparing to unpack .../libgtk-3-bin_3.18.9-1ubuntu3.3_amd64.deb ...
Adding 'diversion of /usr/sbin/update-icon-caches to /usr/sbin/update-icon-caches.gtk2 by libgtk-3-bin'
Adding 'diversion of /usr/share/man/man8/update-icon-caches.8.gz to /usr/share/man/man8/update-icon-caches.gtk2.8.gz by libgtk-3-bin'
Selecting previously unselected package libcroco3:amd64.
Preparing to unpack .../libcroco3_0.6.11-1_amd64.deb ...
Unpacking libcroco3:amd64 (0.6.11-1) ...
Selecting previously unselected package librsvg2-2:amd64.
---
Unpacking libxxf86dga1:amd64 (2:1.1.4-1) ...
Selecting previously unselected package libxxf86vm1:amd64.
Preparing to unpack .../libxxf86vm1_1%3a1.1.4-1_amd64.deb ...
Unpacking libxxf86vm1:amd64 (1:1.1.4-1) ...
Selecting previously unselected package libtxc-dxtn-s2tc0:amd64.
Preparing to unpack .../libtxc-dxtn-s2tc0_0~git20131104-1.1_amd64.deb ...
Unpacking libtxc-dxtn-s2tc0:amd64 (0~git20131104-1.1) ...
Preparing to unpack .../distro-info-data_0.28ubuntu0.17_all.deb ...
Unpacking distro-info-data (0.28ubuntu0.17) ...
Selecting previously unselected package iproute2.
Preparing to unpack .../iproute2_4.3.0-1ubuntu3.16.04.5_amd64.deb ...
---
Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
Setting up libsensors4:amd64 (1:3.4.0-2) ...
Setting up libgl1-mesa-dri:amd64 (18.0.5-0ubuntu0~16.04.1) ...
Setting up libegl1-mesa:amd64 (18.0.5-0ubuntu0~16.04.1) ...
update-alternatives: using /usr/lib/x86_64-linux-gnu/mesa-egl/ld.so.conf to provide /etc/ld.so.conf.d/x86_64-linux-gnu_EGL.conf (x86_64-linux-gnu_egl_conf) in auto mode
Setting up x11-common (1:7.7+13ubuntu3.1) ...
debconf: unable to initialize frontend: Dialog
debconf: (TERM is not set, so the dialog frontend is not usable.)
debconf: falling back to frontend: Readline
---
Setting up libxtst6:amd64 (2:1.2.2-1) ...
Setting up libxv1:amd64 (2:1.0.10-1) ...
Setting up libxxf86dga1:amd64 (2:1.1.4-1) ...
Setting up libxxf86vm1:amd64 (1:1.1.4-1) ...
Setting up libtxc-dxtn-s2tc0:amd64 (0~git20131104-1.1) ...
update-alternatives: using /usr/lib/x86_64-linux-gnu/libtxc_dxtn_s2tc.so.0 to provide /usr/lib/x86_64-linux-gnu/libtxc_dxtn.so (libtxc-dxtn-x86_64-linux-gnu) in auto mode
Setting up iproute2 (4.3.0-1ubuntu3.16.04.5) ...
Setting up ifupdown (0.8.10ubuntu1.4) ...
Creating /etc/network/interfaces.
Setting up libisc-export160 (1:9.10.3.dfsg.P4-8ubuntu1.19) ...
---
Setting up x11-xserver-utils (7.7+7) ...
Setting up xdg-utils (1.1.1-1ubuntu1.16.04.5) ...
Setting up libauthen-sasl-perl (2.1600-1) ...
Setting up libgtk-3-bin (3.18.9-1ubuntu3.3) ...
Setting up adwaita-icon-theme (3.18.0-2ubuntu3.1) ...
update-alternatives: using /usr/share/icons/Adwaita/cursor.theme to provide /usr/share/icons/default/index.theme (x-cursor-theme) in auto mode
Setting up humanity-icon-theme (0.6.10.1) ...
Setting up gconf-service (3.2.6-3ubuntu6) ...
Setting up libnss3-nssdb (2:3.28.4-0ubuntu0.16.04.14) ...
Setting up libnss3:amd64 (2:3.28.4-0ubuntu0.16.04.14) ...
Setting up ubuntu-mono (14.04+16.04.20180326-0ubuntu1) ...
Setting up libgtk-3-common (3.18.9-1ubuntu3.3) ...
---
 ---> 3f7bdcffdf62
Successfully built 3f7bdcffdf62
Successfully tagged rust-ci:latest
Built container sha256:3f7bdcffdf62a411af2543d88ff4915197da59360126b69461c5e5dce02a1946
Uploading finished image to https://ci-caches.rust-lang.org/docker/8b52bdaa11f60b8ebeeee195d21b16ee164e199c8349951039049b9f1c06e3e4763d0e580d3716646b8e7983fa162aadff0b80e5ac30f21fd5f7c8f8e3f1cb20
upload failed: - to s3://rust-lang-ci-sccache2/docker/8b52bdaa11f60b8ebeeee195d21b16ee164e199c8349951039049b9f1c06e3e4763d0e580d3716646b8e7983fa162aadff0b80e5ac30f21fd5f7c8f8e3f1cb20 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-tools]
---
theme-change... ok
toggle-docs-mobile... ok
toggle-docs... ok
toggled-open-implementations... FAILED
[ERROR] line 5: expected only a CSS selector or an XPath in the tuple, found 3 elements



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui" "--tests-folder" "/checkout/src/test/rustdoc-gui"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/rustdoc-gui --stage 2
Build completed unsuccessfully in 0:00:35
