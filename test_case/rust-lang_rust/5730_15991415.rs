
%:
    dh $@ 

override_dh_auto_configure:
    ./configure --prefix=/usr
