plain
[00:05:57]   libx32gcc1 libx32gomp1 libx32itm1 libx32quadmath0 libx32stdc++6 libx32ubsan0
[00:05:57]   pinentry-curses python-apt-common python3-apt python3-dbus python3-gi
[00:05:57]   python3-software-properties
[00:05:57] Suggested packages:
[00:05:57]   dbus-user-session libpam-systemd pinentry-gnome3 tor scdaemon isoquery
[00:05:57]   gmp-doc libgmp10-doc libmpfr-doc pinentry-doc python3-apt-dbg python-apt-doc
[00:05:57] Recommended packages:
[00:05:57]   gnupg dbus nodejs-doc unattended-upgrades
[00:05:57] The following NEW packages will be installed:
[00:05:57]   dirmngr gcc-7-multilib gcc-multilib gir1.2-glib-2.0 gpg gpg-agent gpgconf
---
[00:06:08] Setting up python3-dbus (1.2.6-1) ...
[00:06:08] Setting up libx32cilkrts5 (7.4.0-1ubuntu1~18.04.1) ...
[00:06:08] Setting up nodejs (8.10.0~dfsg-2ubuntu0.4) ...
[00:06:08] update-alternatives: using /usr/bin/nodejs to provide /usr/bin/js (js) in auto mode
[00:06:08] update-alternatives: warning: skip creation of /usr/share/man/man1/js.1.gz because associated file /usr/share/man/man1/nodejs.1.gz (of link group js) doesn't exist
[00:06:08] Setting up libx32ubsan0 (7.4.0-1ubuntu1~18.04.1) ...
[00:06:08] Setting up gpgconf (2.2.4-1ubuntu1.2) ...
[00:06:08] Setting up lib32asan4 (7.4.0-1ubuntu1~18.04.1) ...
[00:06:08] Setting up lib32mpx2 (8.3.0-6ubuntu1~18.04.1) ...
---
[00:06:12]  ---> f5673db17933
[00:06:12] Step 6/40 : RUN apt-key adv --batch --yes --keyserver keyserver.ubuntu.com --recv-keys 74DA7924C5513486
[00:06:12]  ---> Running in e37cf6af88e4
[00:06:13] Warning: apt-key output should not be parsed (stdout is not a terminal)
[00:06:13] Executing: /tmp/apt-key-gpghome.xWJBx69nEr/gpg.1.sh --batch --yes --keyserver keyserver.ubuntu.com --recv-keys 74DA7924C5513486
[00:06:13] gpg: Total number processed: 1
[00:06:13] gpg:               imported: 1
[00:06:14] Removing intermediate container e37cf6af88e4
[00:06:14]  ---> 6837a880fcf7
