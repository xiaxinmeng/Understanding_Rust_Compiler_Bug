 shell
  CFLAGS="${CFLAGS:--O2 -g -pipe -Wall -Werror=format-security -Wp,-D_FORTIFY_SOURCE=2 -fexceptions -fstack-protector-strong --param=ssp-buffer-size=4 -grecord-gcc-switches -specs=/usr/lib/rpm/redhat/redhat-hardened-cc1 -m64 -mtune=generic}" ; export CFLAGS ; 
  CXXFLAGS="${CXXFLAGS:--O2 -g -pipe -Wall -Werror=format-security -Wp,-D_FORTIFY_SOURCE=2 -fexceptions -fstack-protector-strong --param=ssp-buffer-size=4 -grecord-gcc-switches -specs=/usr/lib/rpm/redhat/redhat-hardened-cc1 -m64 -mtune=generic}" ; export CXXFLAGS ; 
  FFLAGS="${FFLAGS:--O2 -g -pipe -Wall -Werror=format-security -Wp,-D_FORTIFY_SOURCE=2 -fexceptions -fstack-protector-strong --param=ssp-buffer-size=4 -grecord-gcc-switches -specs=/usr/lib/rpm/redhat/redhat-hardened-cc1 -m64 -mtune=generic -I/usr/lib64/gfortran/modules}" ; export FFLAGS ; 
  FCFLAGS="${FCFLAGS:--O2 -g -pipe -Wall -Werror=format-security -Wp,-D_FORTIFY_SOURCE=2 -fexceptions -fstack-protector-strong --param=ssp-buffer-size=4 -grecord-gcc-switches -specs=/usr/lib/rpm/redhat/redhat-hardened-cc1 -m64 -mtune=generic -I/usr/lib64/gfortran/modules}" ; export FCFLAGS ; 
  LDFLAGS="${LDFLAGS:--Wl,-z,relro -specs=/usr/lib/rpm/redhat/redhat-hardened-ld}"; export LDFLAGS; 
  [ "1" = 1 ] && for i in $(find $(dirname ./configure) -name config.guess -o -name config.sub) ; do 
      [ -f /usr/lib/rpm/redhat/$(basename $i) ] && /usr/bin/rm -f $i && /usr/bin/cp -fv /usr/lib/rpm/redhat/$(basename $i) $i ; 
  done ; 
  [ "1" = 1 ] && [ x != "x-specs=/usr/lib/rpm/redhat/redhat-hardened-ld" ] && 
      for i in $(find . -name ltmain.sh) ; do 
        /usr/bin/sed -i.backup -e 's~compiler_flags=$~compiler_flags="-specs=/usr/lib/rpm/redhat/redhat-hardened-ld"~' $i 
      done ; 
  ./configure --build=x86_64-redhat-linux-gnu --host=x86_64-redhat-linux-gnu \
    --program-prefix= \
    --disable-dependency-tracking \
    --prefix=/usr \
    --exec-prefix=/usr \
    --bindir=/usr/bin \
    --sbindir=/usr/sbin \
    --sysconfdir=/etc \
    --datadir=/usr/share \
    --includedir=/usr/include \
    --libdir=/usr/lib64 \
    --libexecdir=/usr/libexec \
    --localstatedir=/var \
    --sharedstatedir=/var/lib \
    --mandir=/usr/share/man \
    --infodir=/usr/share/info
