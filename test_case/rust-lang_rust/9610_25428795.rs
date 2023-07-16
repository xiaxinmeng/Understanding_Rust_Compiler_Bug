
tar zxvf /usr/portage/distfiles/rust-0.8.tar.gz;
cd rust-0.8/;
mkdir dl/;
ln -s /usr/portage/distfiles/rust-stage0-2013-09-23-348d844-linux-x86_64-47906010eb676cbf9e0caa0773d9ef2dce89e9f8.tar.bz2 dl/;
./configure;
make -j1
