sh
gdb --version     # GNU gdb (GDB) Fedora  10.1-2.fc33
./x.py test          # those 4 tests failed
sudo dnf downgrade -y gdb
gdb --version    # GNU gdb (GDB) Fedora 9.2-7.fc33
./x.py test          # all tests ok
