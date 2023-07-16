 bash
sudo mkdir -p /usr/local/stow/rust-git
mkdir inst
make install DESTDIR="`pwd`/inst"
sudo cp -r ./inst/usr/local/* /usr/local/stow/rust-git
(cd /usr/local/stow; sudo stow rust-git)
