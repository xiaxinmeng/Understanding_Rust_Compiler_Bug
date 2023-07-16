plain
 23  506M   23  118M    0     0   146M      0  0:00:03 --:--:--  0:00:03  146M
 56  506M   56  285M    0     0   157M      0  0:00:03  0:00:01  0:00:02  157M
 88  506M   88  449M    0     0   160M      0  0:00:03  0:00:02  0:00:01  159M
100  506M  100  506M    0     0   159M      0  0:00:03  0:00:03 --:--:--  159M
+ unzip -q android-ndk-r25b-linux.zip
+ rm android-ndk-r25b-linux.zip
+ mv android-ndk-r25b ndk
 ---> efee734296e9
Step 7/21 : RUN dpkg --add-architecture i386 &&     apt-get update &&     apt-get install -y --no-install-recommends   libgl1-mesa-glx   libpulse0   libstdc++6:i386   openjdk-8-jre-headless   tzdata   wget   python3
 ---> Running in da94afeda9ac
Hit:1 http://security.ubuntu.com/ubuntu jammy-security InRelease
---
  opus-tools pcscd pulseaudio lm-sensors libnss-mdns fonts-dejavu-extra
  fonts-ipafont-gothic fonts-ipafont-mincho fonts-wqy-microhei
  fonts-wqy-zenhei fonts-indic
Recommended packages:
  libidn2-0:i386 libnss-nis:i386 libnss-nisplus:i386 dbus libgl1-amber-dri
The following NEW packages will be installed:
  ca-certificates-java fontconfig-config fonts-dejavu-core gcc-12-base:i386
  java-common libapparmor1 libasyncns0 libavahi-client3 libavahi-common-data
  libavahi-common3 libbsd0 libc6:i386 libcrypt1:i386 libcups2 libdbus-1-3
---
Setting up libgl1-mesa-glx:amd64 (22.0.5-0ubuntu0.1) ...
Setting up libcrypt1:i386 (1:4.4.27-1) ...
Setting up libgcc-s1:i386 (12.1.0-2ubuntu1~22.04) ...
Setting up libc6:i386 (2.35-0ubuntu3.1) ...
Setting up ca-certificates-java (20190909) ...
head: cannot open '/etc/ssl/certs/java/cacerts' for reading: No such file or directory
Adding debian:CA_Disig_Root_R2.pem
Adding debian:SwissSign_Gold_CA_-_G2.pem
Adding debian:EC-ACC.pem
Adding debian:AffirmTrust_Premium_ECC.pem
---
Adding debian:Entrust_Root_Certification_Authority_-_G2.pem
Adding debian:ANF_Secure_Server_Root_CA.pem
Adding debian:Buypass_Class_2_Root_CA.pem
Adding debian:TrustCor_RootCert_CA-2.pem
Adding debian:AC_RAIZ_FNMT-RCM_SERVIDORES_SEGUROS.pem
Adding debian:IdenTrust_Commercial_Root_CA_1.pem
Adding debian:T-TeleSec_GlobalRoot_Class_2.pem
Adding debian:TWCA_Global_Root_CA.pem
Adding debian:E-Tugra_Certification_Authority.pem
---
Adding debian:CFCA_EV_ROOT.pem
Adding debian:Entrust_Root_Certification_Authority_-_EC1.pem
Adding debian:Starfield_Class_2_CA.pem
Adding debian:TWCA_Root_Certification_Authority.pem
Adding debian:Certum_EC-384_CA.pem
Adding debian:QuoVadis_Root_CA_3_G3.pem
Adding debian:QuoVadis_Root_CA_2.pem
Adding debian:UCA_Extended_Validation_Root.pem
Adding debian:Staat_der_Nederlanden_EV_Root_CA.pem
---
Successfully built 276e66bb58a7
Successfully tagged rust-ci:latest
Built container sha256:276e66bb58a73d95cf37a89fdc0e63eaa3c6891b5f53a1fa1ccdabe898168517
Uploading finished image to https://ci-caches.rust-lang.org/docker/757fc9f782f69e21d7164b87e85f90996325eb3bdbcf2bff02e923081f69f382160aadd7b85c216e92c9a0ff814b7972478800996600d8eb4ddbbf70777a6e72
upload failed: - to s3://rust-lang-ci-sccache2/docker/757fc9f782f69e21d7164b87e85f90996325eb3bdbcf2bff02e923081f69f382160aadd7b85c216e92c9a0ff814b7972478800996600d8eb4ddbbf70777a6e72 Unable to locate credentials
+ exec /checkout/src/ci/run.sh
+ nohup nohup emulator @armeabi-v7a-18 -engine classic -no-window -partition-size 2047
+ 
[CI_JOB_NAME=arm-android]
