
bisecting ci builds
starting at 0eb878d2aa6e3a1cb315f3f328681b26bb4bffdb, ending at origin/master
fetching commits from 0eb878d2aa6e3a1cb315f3f328681b26bb4bffdb to origin/master
opening existing repository at "/home/jon/dev/others/rust"
refreshing repository
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 25 bors merge commits in the specified range
opening existing repository at "/home/jon/dev/others/rust"
refreshing repository
validated commits found, specifying toolchains
testing commits
verifying the start of the range does not reproduce the regression
installing 0eb878d2aa6e3a1cb315f3f328681b26bb4bffdb
std for x86_64-unknown-linux-gnu: 17.53 MB / 17.53 MB [======================================================================================================================================================================================================================] 100.00 % 679.77 KB/s 0s testing 0eb878d2aa6e3a1cb315f3f328681b26bb4bffdb
tested 0eb878d2aa6e3a1cb315f3f328681b26bb4bffdb, got No
uninstalling 0eb878d2aa6e3a1cb315f3f328681b26bb4bffdb
confirmed the start of the range does not reproduce the regression
verifying the end of the range reproduces the regression
installing 3c5b1b7d63f55ac96fc7cd06df01e0f0e4f49d47
std for x86_64-unknown-linux-gnu: 17.57 MB / 17.57 MB [=========================================================================================================================================================================================================================] 100.00 % 939.29 KB/s testing 3c5b1b7d63f55ac96fc7cd06df01e0f0e4f49d47
tested 3c5b1b7d63f55ac96fc7cd06df01e0f0e4f49d47, got Yes
uninstalling 3c5b1b7d63f55ac96fc7cd06df01e0f0e4f49d47
confirmed the end of the range reproduces the regression
installing 360e42de82152c4e1a6e70d2f228dd3748c50c8d
std for x86_64-unknown-linux-gnu: 17.62 MB / 17.62 MB [=========================================================================================================================================================================================================================] 100.00 % 749.47 KB/s testing 360e42de82152c4e1a6e70d2f228dd3748c50c8d
tested 360e42de82152c4e1a6e70d2f228dd3748c50c8d, got Yes
uninstalling 360e42de82152c4e1a6e70d2f228dd3748c50c8d
installing 4f0edbdfe5f111c43a5e06f68186b95141d1f6c8
std for x86_64-unknown-linux-gnu: 17.57 MB / 17.57 MB [=========================================================================================================================================================================================================================] 100.00 % 953.92 KB/s testing 4f0edbdfe5f111c43a5e06f68186b95141d1f6c8
tested 4f0edbdfe5f111c43a5e06f68186b95141d1f6c8, got Yes
uninstalling 4f0edbdfe5f111c43a5e06f68186b95141d1f6c8
installing 04e7f96dd89b1f0ad615dff1c85d11d4c4c64cb4
std for x86_64-unknown-linux-gnu: 17.54 MB / 17.54 MB [=========================================================================================================================================================================================================================] 100.00 % 708.05 KB/s testing 04e7f96dd89b1f0ad615dff1c85d11d4c4c64cb4
tested 04e7f96dd89b1f0ad615dff1c85d11d4c4c64cb4, got Yes
uninstalling 04e7f96dd89b1f0ad615dff1c85d11d4c4c64cb4
installing 5703b7aafb70e77547e8f03876a5911a2e89a2a5
std for x86_64-unknown-linux-gnu: 17.57 MB / 17.57 MB [=========================================================================================================================================================================================================================] 100.00 % 906.34 KB/s testing 5703b7aafb70e77547e8f03876a5911a2e89a2a5
tested 5703b7aafb70e77547e8f03876a5911a2e89a2a5, got No
uninstalling 5703b7aafb70e77547e8f03876a5911a2e89a2a5
installing 55aee8d49628ae8218e91745c388d5dc36771248
std for x86_64-unknown-linux-gnu: 17.50 MB / 17.50 MB [===========================================================================================================================================================================================================================] 100.00 % 4.52 MB/s testing 55aee8d49628ae8218e91745c388d5dc36771248
tested 55aee8d49628ae8218e91745c388d5dc36771248, got Yes
uninstalling 55aee8d49628ae8218e91745c388d5dc36771248
searched toolchains 0eb878d2aa6e3a1cb315f3f328681b26bb4bffdb through 3c5b1b7d63f55ac96fc7cd06df01e0f0e4f49d47
regression in 55aee8d49628ae8218e91745c388d5dc36771248
