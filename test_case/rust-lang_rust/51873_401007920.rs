
➜  ~ dnf info mariadb-devel libsq3-devel
Last metadata expiration check: 29 days, 11:57:20 ago on Tue 29 May 2018 07:44:41 PM EDT.
Available Packages
Name         : libsq3-devel
Version      : 20071018
Release      : 23.fc28
Arch         : i686
Size         : 181 k
Source       : libsqlite3x-20071018-23.fc28.src.rpm
Repo         : fedora
Summary      : Development files for libsqlite3x
URL          : http://www.wanderinghorse.net/computing/sqlite/
License      : zlib
Description  : The libsq3-devel package contains libraries and header files for
             : developing applications that use libsq3.

Name         : libsq3-devel
Version      : 20071018
Release      : 23.fc28
Arch         : x86_64
Size         : 181 k
Source       : libsqlite3x-20071018-23.fc28.src.rpm
Repo         : fedora
Summary      : Development files for libsqlite3x
URL          : http://www.wanderinghorse.net/computing/sqlite/
License      : zlib
Description  : The libsq3-devel package contains libraries and header files for
             : developing applications that use libsq3.

Name         : mariadb-devel
Epoch        : 3
Version      : 10.2.14
Release      : 1.fc28
Arch         : i686
Size         : 976 k
Source       : mariadb-10.2.14-1.fc28.src.rpm
Repo         : fedora
Summary      : Files for development of MariaDB/MySQL applications
URL          : http://mariadb.org
License      : GPLv2 with exceptions and LGPLv2 and BSD
Description  : MariaDB is a multi-user, multi-threaded SQL database server.
             : MariaDB is a community developed branch of MySQL.
             : 
             : 
             : 
             : 
             : This package contains everything needed for developing MariaDB/MySQL server
             : applications. For developing client applications, use mariadb-connector-c
             : package.

Name         : mariadb-devel
Epoch        : 3
Version      : 10.2.14
Release      : 1.fc28
Arch         : x86_64
Size         : 976 k
Source       : mariadb-10.2.14-1.fc28.src.rpm
Repo         : fedora
Summary      : Files for development of MariaDB/MySQL applications
URL          : http://mariadb.org
License      : GPLv2 with exceptions and LGPLv2 and BSD
Description  : MariaDB is a multi-user, multi-threaded SQL database server.
             : MariaDB is a community developed branch of MySQL.
             : 
             : 
             : 
             : 
             : This package contains everything needed for developing MariaDB/MySQL server
             : applications. For developing client applications, use mariadb-connector-c
             : package.

➜  ~ sudo dnf install mariadb-devel libsq3-devel
[sudo] password for kus: 
Last metadata expiration check: 1:10:36 ago on Thu 28 Jun 2018 06:31:44 AM EDT.
Dependencies resolved.
===============================================================================================================================================================================================================================================================================
 Package                                                                      Arch                                                      Version                                                               Repository                                                  Size
===============================================================================================================================================================================================================================================================================
Installing:
 libsq3-devel                                                                 x86_64                                                    20071018-23.fc28                                                      fedora                                                     181 k
 mariadb-devel                                                                x86_64                                                    3:10.2.15-2.fc28                                                      updates                                                    977 k
Installing dependencies:
 keyutils-libs-devel                                                          x86_64                                                    1.5.10-6.fc28                                                         fedora                                                      47 k
 krb5-devel                                                                   x86_64                                                    1.16.1-7.fc28                                                         updates                                                    544 k
 libcom_err-devel                                                             x86_64                                                    1.43.8-2.fc28                                                         fedora                                                      36 k
 libkadm5                                                                     x86_64                                                    1.16.1-7.fc28                                                         updates                                                    181 k
 libselinux-devel                                                             x86_64                                                    2.8-1.fc28                                                            updates                                                    197 k
 libsepol-devel                                                               x86_64                                                    2.8-1.fc28                                                            updates                                                     84 k
 libsq3                                                                       x86_64                                                    20071018-23.fc28                                                      fedora                                                      44 k
 libverto-devel                                                               x86_64                                                    0.3.0-5.fc28                                                          fedora                                                      17 k
 mariadb-connector-c-devel                                                    x86_64                                                    3.0.5-1.fc28                                                          updates                                                     61 k
 openssl-devel                                                                x86_64                                                    1:1.1.0h-3.fc28                                                       fedora                                                     1.9 M
 pcre2-devel                                                                  x86_64                                                    10.31-4.fc28                                                          fedora                                                     588 k
 pcre2-utf32                                                                  x86_64                                                    10.31-4.fc28                                                          fedora                                                     212 k
 sqlite-devel                                                                 x86_64                                                    3.22.0-4.fc28                                                         fedora                                                     153 k
 zlib-devel                                                                   x86_64                                                    1.2.11-8.fc28                                                         updates                                                     55 k

Transaction Summary
===============================================================================================================================================================================================================================================================================
Install  16 Packages

Total download size: 5.2 M
Installed size: 14 M
Is this ok [y/N]: y
Downloading Packages:
(1/16): libsq3-20071018-23.fc28.x86_64.rpm                                                                                                                                                                                                     216 kB/s |  44 kB     00:00    
(2/16): libsq3-devel-20071018-23.fc28.x86_64.rpm                                                                                                                                                                                               719 kB/s | 181 kB     00:00    
(3/16): sqlite-devel-3.22.0-4.fc28.x86_64.rpm                                                                                                                                                                                                  1.0 MB/s | 153 kB     00:00    
(4/16): openssl-devel-1.1.0h-3.fc28.x86_64.rpm                                                                                                                                                                                                 3.4 MB/s | 1.9 MB     00:00    
(5/16): mariadb-connector-c-devel-3.0.5-1.fc28.x86_64.rpm                                                                                                                                                                                       81 kB/s |  61 kB     00:00    
(6/16): zlib-devel-1.2.11-8.fc28.x86_64.rpm                                                                                                                                                                                                    513 kB/s |  55 kB     00:00    
(7/16): mariadb-devel-10.2.15-2.fc28.x86_64.rpm                                                                                                                                                                                                852 kB/s | 977 kB     00:01    
(8/16): libselinux-devel-2.8-1.fc28.x86_64.rpm                                                                                                                                                                                                 1.5 MB/s | 197 kB     00:00    
(9/16): libkadm5-1.16.1-7.fc28.x86_64.rpm                                                                                                                                                                                                      699 kB/s | 181 kB     00:00    
(10/16): libsepol-devel-2.8-1.fc28.x86_64.rpm                                                                                                                                                                                                  766 kB/s |  84 kB     00:00    
(11/16): keyutils-libs-devel-1.5.10-6.fc28.x86_64.rpm                                                                                                                                                                                          1.0 MB/s |  47 kB     00:00    
(12/16): libcom_err-devel-1.43.8-2.fc28.x86_64.rpm                                                                                                                                                                                             816 kB/s |  36 kB     00:00    
(13/16): libverto-devel-0.3.0-5.fc28.x86_64.rpm                                                                                                                                                                                                429 kB/s |  17 kB     00:00    
(14/16): pcre2-utf32-10.31-4.fc28.x86_64.rpm                                                                                                                                                                                                   2.3 MB/s | 212 kB     00:00    
(15/16): pcre2-devel-10.31-4.fc28.x86_64.rpm                                                                                                                                                                                                   2.3 MB/s | 588 kB     00:00    
(16/16): krb5-devel-1.16.1-7.fc28.x86_64.rpm                                                                                                                                                                                                   571 kB/s | 544 kB     00:00    
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
Total                                                                                                                                                                                                                                          2.1 MB/s | 5.2 MB     00:02     
Running transaction check
Transaction check succeeded.
Running transaction test
Transaction test succeeded.
Running transaction
  Preparing        :                                                                                                                                                                                                                                                       1/1 
  Installing       : pcre2-utf32-10.31-4.fc28.x86_64                                                                                                                                                                                                                      1/16 
  Installing       : pcre2-devel-10.31-4.fc28.x86_64                                                                                                                                                                                                                      2/16 
  Installing       : libverto-devel-0.3.0-5.fc28.x86_64                                                                                                                                                                                                                   3/16 
  Installing       : libcom_err-devel-1.43.8-2.fc28.x86_64                                                                                                                                                                                                                4/16 
  Installing       : keyutils-libs-devel-1.5.10-6.fc28.x86_64                                                                                                                                                                                                             5/16 
  Installing       : libsepol-devel-2.8-1.fc28.x86_64                                                                                                                                                                                                                     6/16 
  Installing       : libselinux-devel-2.8-1.fc28.x86_64                                                                                                                                                                                                                   7/16 
  Installing       : libkadm5-1.16.1-7.fc28.x86_64                                                                                                                                                                                                                        8/16 
  Installing       : krb5-devel-1.16.1-7.fc28.x86_64                                                                                                                                                                                                                      9/16 
  Installing       : zlib-devel-1.2.11-8.fc28.x86_64                                                                                                                                                                                                                     10/16 
  Installing       : openssl-devel-1:1.1.0h-3.fc28.x86_64                                                                                                                                                                                                                11/16 
  Installing       : mariadb-connector-c-devel-3.0.5-1.fc28.x86_64                                                                                                                                                                                                       12/16 
  Installing       : sqlite-devel-3.22.0-4.fc28.x86_64                                                                                                                                                                                                                   13/16 
  Installing       : libsq3-20071018-23.fc28.x86_64                                                                                                                                                                                                                      14/16 
  Running scriptlet: libsq3-20071018-23.fc28.x86_64                                                                                                                                                                                                                      14/16 
  Installing       : libsq3-devel-20071018-23.fc28.x86_64                                                                                                                                                                                                                15/16 
  Installing       : mariadb-devel-3:10.2.15-2.fc28.x86_64                                                                                                                                                                                                               16/16 
  Running scriptlet: mariadb-devel-3:10.2.15-2.fc28.x86_64                                                                                                                                                                                                               16/16 
  Verifying        : mariadb-devel-3:10.2.15-2.fc28.x86_64                                                                                                                                                                                                                1/16 
  Verifying        : libsq3-devel-20071018-23.fc28.x86_64                                                                                                                                                                                                                 2/16 
  Verifying        : libsq3-20071018-23.fc28.x86_64                                                                                                                                                                                                                       3/16 
  Verifying        : sqlite-devel-3.22.0-4.fc28.x86_64                                                                                                                                                                                                                    4/16 
  Verifying        : mariadb-connector-c-devel-3.0.5-1.fc28.x86_64                                                                                                                                                                                                        5/16 
  Verifying        : openssl-devel-1:1.1.0h-3.fc28.x86_64                                                                                                                                                                                                                 6/16 
  Verifying        : krb5-devel-1.16.1-7.fc28.x86_64                                                                                                                                                                                                                      7/16 
  Verifying        : zlib-devel-1.2.11-8.fc28.x86_64                                                                                                                                                                                                                      8/16 
  Verifying        : libkadm5-1.16.1-7.fc28.x86_64                                                                                                                                                                                                                        9/16 
  Verifying        : libselinux-devel-2.8-1.fc28.x86_64                                                                                                                                                                                                                  10/16 
  Verifying        : libsepol-devel-2.8-1.fc28.x86_64                                                                                                                                                                                                                    11/16 
  Verifying        : keyutils-libs-devel-1.5.10-6.fc28.x86_64                                                                                                                                                                                                            12/16 
  Verifying        : libcom_err-devel-1.43.8-2.fc28.x86_64                                                                                                                                                                                                               13/16 
  Verifying        : libverto-devel-0.3.0-5.fc28.x86_64                                                                                                                                                                                                                  14/16 
  Verifying        : pcre2-devel-10.31-4.fc28.x86_64                                                                                                                                                                                                                     15/16 
  Verifying        : pcre2-utf32-10.31-4.fc28.x86_64                                                                                                                                                                                                                     16/16 

Installed:
  libsq3-devel.x86_64 20071018-23.fc28 mariadb-devel.x86_64 3:10.2.15-2.fc28 keyutils-libs-devel.x86_64 1.5.10-6.fc28 krb5-devel.x86_64 1.16.1-7.fc28               libcom_err-devel.x86_64 1.43.8-2.fc28 libkadm5.x86_64 1.16.1-7.fc28   libselinux-devel.x86_64 2.8-1.fc28
  libsepol-devel.x86_64 2.8-1.fc28     libsq3.x86_64 20071018-23.fc28        libverto-devel.x86_64 0.3.0-5.fc28       mariadb-connector-c-devel.x86_64 3.0.5-1.fc28 openssl-devel.x86_64 1:1.1.0h-3.fc28  pcre2-devel.x86_64 10.31-4.fc28 pcre2-utf32.x86_64 10.31-4.fc28   
  sqlite-devel.x86_64 3.22.0-4.fc28    zlib-devel.x86_64 1.2.11-8.fc28      

Complete!
