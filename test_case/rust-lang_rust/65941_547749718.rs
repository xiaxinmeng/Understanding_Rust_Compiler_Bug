plain
2019-10-30T05:41:17.1257919Z  55  813M   55  449M    0     0  2248k      0  0:06:10  0:03:24  0:02:46 2961k
2019-10-30T05:41:17.1258488Z  55  813M   55  449M    0     0  2248k      0  0:06:10  0:03:24  0:02:46 2760k
2019-10-30T05:41:17.1326266Z curl: (56) GnuTLS recv error (-24): Decryption has failed.
2019-10-30T05:41:17.1335164Z The command has failed after 5 attempts.
2019-10-30T05:42:20.1062513Z Error processing tar file(exit status 1): unexpected EOF
2019-10-30T05:42:20.1137460Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/arm-android/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-10-30T05:42:20.3024117Z Sending build context to Docker daemon  524.3kB
2019-10-30T05:42:20.3024194Z 
2019-10-30T05:42:20.3269235Z Step 1/20 : FROM ubuntu:16.04
---
2019-10-30T05:44:32.9987872Z   0  929M    0   840    0     0   1560      0   7d 05h --:--:--   7d 05h  1558
2019-10-30T05:44:33.9998653Z   0  929M    0 8399k    0     0  5755k      0  0:02:45  0:00:01  0:02:44 5753k
2019-10-30T05:44:34.3796131Z   2  929M    2 20.9M    0     0  8719k      0  0:01:49  0:00:02  0:01:47 8718k
2019-10-30T05:44:34.3796307Z   2  929M    2 23.4M    0     0  8456k      0  0:01:52  0:00:02  0:01:50 8456k
2019-10-30T05:44:34.3880676Z curl: (56) GnuTLS recv error (-24): Decryption has failed.
2019-10-30T05:44:34.3903479Z + unzip -q android-ndk-r15c-linux-x86_64.zip
2019-10-30T05:44:34.3927926Z [android-ndk-r15c-linux-x86_64.zip]
2019-10-30T05:44:34.3929534Z   End-of-central-directory signature not found.  Either this file is not
2019-10-30T05:44:34.3930675Z   a zipfile, or it constitutes one disk of a multi-part archive.  In the
2019-10-30T05:44:34.3931005Z   latter case the central directory and zipfile comment will be found on
2019-10-30T05:44:34.3931377Z   the last disk(s) of this archive.
2019-10-30T05:44:34.3932055Z unzip:  cannot find zipfile directory in one of android-ndk-r15c-linux-x86_64.zip or
2019-10-30T05:44:34.3932544Z         android-ndk-r15c-linux-x86_64.zip.zip, and cannot find android-ndk-r15c-linux-x86_64.zip.ZIP, period.
2019-10-30T05:44:34.3933783Z + rm android-ndk-r15c-linux-x86_64.zip
2019-10-30T05:44:34.3973753Z + mv android-ndk-* ndk
2019-10-30T05:44:34.3982411Z mv: cannot stat 'android-ndk-*': No such file or directory
2019-10-30T05:44:34.7828419Z The command '/bin/sh -c . /scripts/android-ndk.sh &&     download_and_make_toolchain android-ndk-r15c-linux-x86_64.zip arm 14' returned a non-zero code: 1
2019-10-30T05:44:35.9110831Z Sending build context to Docker daemon  524.3kB
2019-10-30T05:44:35.9111042Z 
2019-10-30T05:44:35.9306584Z Step 1/20 : FROM ubuntu:16.04
2019-10-30T05:44:35.9308415Z  ---> b9409899fe86
---
2019-10-30T05:47:48.6706281Z   libxcb-sync1 libxcb1 libxdamage1 libxdmcp6 libxext6 libxfixes3 libxi6
2019-10-30T05:47:48.6706532Z   libxrender1 libxshmfence1 libxtst6 libxxf86vm1 python3-minimal python3.5
2019-10-30T05:47:48.6706731Z   python3.5-minimal x11-common
2019-10-30T05:47:48.6706892Z Suggested packages:
2019-10-30T05:47:48.6707152Z   default-jre glibc-doc:i386 locales:i386 cups-common liblcms2-utils pciutils
2019-10-30T05:47:48.6707377Z   pcscd pulseaudio lm-sensors libnss-mdns fonts-dejavu-extra
2019-10-30T05:47:48.6707628Z   fonts-ipafont-gothic fonts-ipafont-mincho ttf-wqy-microhei | ttf-wqy-zenhei
2019-10-30T05:47:48.6708622Z   fonts-indic python3-doc python3-tk python3-venv python3.5-venv python3.5-doc
2019-10-30T05:47:48.6709115Z Recommended packages:
2019-10-30T05:47:48.6709447Z   dbus libtxc-dxtn-s2tc | libtxc-dxtn-s2tc0 | libtxc-dxtn0 tcpd
2019-10-30T05:47:48.6709558Z The following NEW packages will be installed:
2019-10-30T05:47:48.6709852Z   ca-certificates-java dh-python gcc-5-base:i386 gcc-6-base:i386 java-common
---
2019-10-30T05:49:06.2667615Z Setting up x11-common (1:7.7+13ubuntu3.1) ...
2019-10-30T05:49:06.6694063Z debconf: unable to initialize frontend: Dialog
2019-10-30T05:49:06.6694445Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-10-30T05:49:06.6694533Z debconf: falling back to frontend: Readline
2019-10-30T05:49:06.6910464Z update-rc.d: warning: start and stop actions are no longer supported; falling back to defaults
2019-10-30T05:49:06.7308784Z invoke-rc.d: policy-rc.d denied execution of start.
2019-10-30T05:49:06.7512926Z Setting up libxtst6:amd64 (2:1.2.2-1) ...
2019-10-30T05:49:06.8272861Z Setting up libxxf86vm1:amd64 (1:1.1.4-1) ...
2019-10-30T05:49:06.9018747Z Setting up libnspr4:amd64 (2:4.13.1-0ubuntu0.16.04.1) ...
---
2019-10-30T05:49:07.8054550Z debconf: unable to initialize frontend: Dialog
2019-10-30T05:49:07.8054710Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-10-30T05:49:07.8054828Z debconf: falling back to frontend: Readline
2019-10-30T05:49:07.8456463Z 
2019-10-30T05:49:07.8457507Z Current default time zone: 'Etc/UTC'
2019-10-30T05:49:07.8498030Z Local time is now:      Wed Oct 30 05:49:07 UTC 2019.
2019-10-30T05:49:07.8498124Z Universal Time is now:  Wed Oct 30 05:49:07 UTC 2019.
2019-10-30T05:49:07.8498590Z Run 'dpkg-reconfigure tzdata' if you wish to change it.
2019-10-30T05:49:07.9236094Z Setting up gcc-5-base:i386 (5.4.0-6ubuntu1~16.04.11) ...
2019-10-30T05:49:07.9766288Z Setting up libbsd0:amd64 (0.8.2-1) ...
2019-10-30T05:49:08.0214658Z Setting up libdrm-common (2.4.91-2~16.04.1) ...
2019-10-30T05:49:08.0712790Z Setting up libdrm2:amd64 (2.4.91-2~16.04.1) ...
---
2019-10-30T05:49:09.0879370Z Setting up libxcb-glx0:amd64 (1.11.1-1ubuntu1) ...
2019-10-30T05:49:09.1408107Z Setting up libxcb-present0:amd64 (1.11.1-1ubuntu1) ...
2019-10-30T05:49:09.2016541Z Setting up libxcb-sync1:amd64 (1.11.1-1ubuntu1) ...
2019-10-30T05:49:09.2545036Z Setting up libgl1-mesa-glx:amd64 (18.0.5-0ubuntu0~16.04.1) ...
2019-10-30T05:49:09.3135679Z update-alternatives: using /usr/lib/x86_64-linux-gnu/mesa/ld.so.conf to provide /etc/ld.so.conf.d/x86_64-linux-gnu_GL.conf (x86_64-linux-gnu_gl_conf) in auto mode
2019-10-30T05:49:09.4167238Z Setting up libvorbisenc2:amd64 (1.3.5-3ubuntu0.2) ...
2019-10-30T05:49:09.4664519Z Setting up libsndfile1:amd64 (1.0.25-10ubuntu0.16.04.2) ...
2019-10-30T05:49:09.5168058Z Setting up libpulse0:amd64 (1:8.0-0ubuntu3.10) ...
2019-10-30T05:49:09.5867179Z Setting up python3 (3.5.1-3) ...
2019-10-30T05:49:09.5867179Z Setting up python3 (3.5.1-3) ...
2019-10-30T05:49:09.6256755Z running python rtupdate hooks for python3.5...
2019-10-30T05:49:09.8651922Z running python post-rtupdate hooks for python3.5...
2019-10-30T05:49:10.0063390Z Setting up libnss3-nssdb (2:3.28.4-0ubuntu0.16.04.6) ...
2019-10-30T05:49:10.0572052Z Setting up libnss3:amd64 (2:3.28.4-0ubuntu0.16.04.6) ...
2019-10-30T05:49:10.1367466Z Setting up ca-certificates-java (20160321ubuntu1) ...
2019-10-30T05:49:10.7667369Z Adding debian:Swisscom_Root_EV_CA_2.pem
2019-10-30T05:49:10.8194195Z Adding debian:GlobalSign_ECC_Root_CA_-_R4.pem
2019-10-30T05:49:10.8234108Z Adding debian:AddTrust_Qualified_Certificates_Root.pem
2019-10-30T05:49:10.8265682Z Adding debian:DigiCert_Global_Root_G3.pem
2019-10-30T05:49:10.8330873Z Adding debian:Verisign_Class_3_Public_Primary_Certification_Authority_-_G3.pem
2019-10-30T05:49:10.8383827Z Adding debian:EC-ACC.pem
2019-10-30T05:49:10.8421004Z Adding debian:Buypass_Class_3_Root_CA.pem
2019-10-30T05:49:10.8453557Z Adding debian:Visa_eCommerce_Root.pem
2019-10-30T05:49:10.8509507Z Adding debian:TÜBİTAK_UEKAE_Kök_Sertifika_Hizmet_Sağlayıcısı_-_Sürüm_3.pem
2019-10-30T05:49:10.8552395Z Adding debian:QuoVadis_Root_CA_3_G3.pem
2019-10-30T05:49:10.8578194Z Adding debian:Certum_Root_CA.pem
2019-10-30T05:49:10.8696226Z Adding debian:SwissSign_Gold_CA_-_G2.pem
2019-10-30T05:49:10.8795523Z Adding debian:Camerfirma_Chambers_of_Commerce_Root.pem
2019-10-30T05:49:10.8874561Z Adding debian:VeriSign_Class_3_Public_Primary_Certification_Authority_-_G4.pem
2019-10-30T05:49:10.8901223Z Adding debian:CNNIC_ROOT.pem
---
2019-10-30T05:49:10.9370714Z Adding debian:VeriSign_Class_3_Public_Primary_Certification_Authority_-_G5.pem
2019-10-30T05:49:10.9407771Z Adding debian:GeoTrust_Primary_Certification_Authority_-_G2.pem
2019-10-30T05:49:10.9477009Z Adding debian:AddTrust_Public_Services_Root.pem
2019-10-30T05:49:10.9547144Z Adding debian:QuoVadis_Root_CA_3.pem
2019-10-30T05:49:10.9571641Z Adding debian:DigiCert_Assured_ID_Root_G2.pem
2019-10-30T05:49:10.9611379Z Adding debian:USERTrust_RSA_Certification_Authority.pem
2019-10-30T05:49:10.9652274Z Adding debian:IdenTrust_Commercial_Root_CA_1.pem
2019-10-30T05:49:10.9763746Z Adding debian:Chambers_of_Commerce_Root_-_2008.pem
2019-10-30T05:49:10.9790178Z Adding debian:DST_ACES_CA_X6.pem
2019-10-30T05:49:10.9870839Z Adding debian:E-Tugra_Certification_Authority.pem
2019-10-30T05:49:10.9951255Z Adding debian:Buypass_Class_2_Root_CA.pem
2019-10-30T05:49:10.9998525Z Adding debian:Security_Communication_Root_CA.pem
2019-10-30T05:49:11.0028907Z Adding debian:Comodo_Trusted_Services_root.pem
2019-10-30T05:49:11.0120668Z Adding debian:Global_Chambersign_Root_-_2008.pem
2019-10-30T05:49:11.0120668Z Adding debian:Global_Chambersign_Root_-_2008.pem
2019-10-30T05:49:11.0216791Z Adding debian:Security_Communication_RootCA2.pem
2019-10-30T05:49:11.0247350Z Adding debian:Entrust.net_Premium_2048_Secure_Server_CA.pem
2019-10-30T05:49:11.0289137Z Adding debian:COMODO_RSA_Certification_Authority.pem
2019-10-30T05:49:11.0315803Z Adding debian:OISTE_WISeKey_Global_Root_GA_CA.pem
2019-10-30T05:49:11.0347862Z Adding debian:Go_Daddy_Class_2_CA.pem
2019-10-30T05:49:11.0370734Z Adding debian:Baltimore_CyberTrust_Root.pem
2019-10-30T05:49:11.0401021Z Adding debian:Staat_der_Nederlanden_Root_CA_-_G2.pem
2019-10-30T05:49:11.0435063Z Adding debian:ISRG_Root_X1.pem
2019-10-30T05:49:11.0460674Z Adding debian:certSIGN_ROOT_CA.pem
2019-10-30T05:49:11.0510100Z Adding debian:Hellenic_Academic_and_Research_Institutions_RootCA_2011.pem
2019-10-30T05:49:11.0526776Z Adding debian:USERTrust_ECC_Certification_Authority.pem
2019-10-30T05:49:11.0546892Z Adding debian:Atos_TrustedRoot_2011.pem
2019-10-30T05:49:11.0566119Z Adding debian:Hongkong_Post_Root_CA_1.pem
2019-10-30T05:49:11.0592405Z Adding debian:AddTrust_Low-Value_Services_Root.pem
2019-10-30T05:49:11.0607092Z Adding debian:GlobalSign_ECC_Root_CA_-_R5.pem
2019-10-30T05:49:11.0626817Z Adding debian:Amazon_Root_CA_1.pem
2019-10-30T05:49:11.0654443Z Adding debian:Izenpe.com.pem
2019-10-30T05:49:11.0675695Z Adding debian:T-TeleSec_GlobalRoot_Class_2.pem
2019-10-30T05:49:11.0694016Z Adding debian:AffirmTrust_Commercial.pem
2019-10-30T05:49:11.0717590Z Adding debian:Cybertrust_Global_Root.pem
2019-10-30T05:49:11.0743295Z Adding debian:OpenTrust_Root_CA_G1.pem
2019-10-30T05:49:11.0772998Z Adding debian:Taiwan_GRCA.pem
2019-10-30T05:49:11.0807165Z Adding debian:China_Internet_Network_Information_Center_EV_Certificates_Root.pem
2019-10-30T05:49:11.0836118Z Adding debian:QuoVadis_Root_CA_2_G3.pem
2019-10-30T05:49:11.0856763Z Adding debian:OISTE_WISeKey_Global_Root_GB_CA.pem
2019-10-30T05:49:11.0884569Z Adding debian:Certinomis_-_Autorité_Racine.pem
2019-10-30T05:49:11.0911785Z Adding debian:TeliaSonera_Root_CA_v1.pem
2019-10-30T05:49:11.0938637Z Adding debian:QuoVadis_Root_CA_1_G3.pem
2019-10-30T05:49:11.0994043Z Adding debian:Certplus_Class_2_Primary_CA.pem
2019-10-30T05:49:11.0994043Z Adding debian:Certplus_Class_2_Primary_CA.pem
2019-10-30T05:49:11.1018450Z Adding debian:TUBITAK_Kamu_SM_SSL_Kok_Sertifikasi_-_Surum_1.pem
2019-10-30T05:49:11.1042118Z Adding debian:TÜRKTRUST_Elektronik_Sertifika_Hizmet_Sağlayıcısı_H5.pem
2019-10-30T05:49:11.1074882Z Adding debian:OpenTrust_Root_CA_G2.pem
2019-10-30T05:49:11.1120972Z Adding debian:COMODO_Certification_Authority.pem
2019-10-30T05:49:11.1162954Z Adding debian:TWCA_Root_Certification_Authority.pem
2019-10-30T05:49:11.1191158Z Adding debian:IdenTrust_Public_Sector_Root_CA_1.pem
2019-10-30T05:49:11.1216061Z Adding debian:Comodo_AAA_Services_root.pem
2019-10-30T05:49:11.1216061Z Adding debian:Comodo_AAA_Services_root.pem
2019-10-30T05:49:11.1246156Z Adding debian:Staat_der_Nederlanden_Root_CA_-_G3.pem
2019-10-30T05:49:11.1275996Z Adding debian:AC_RAIZ_FNMT-RCM.pem
2019-10-30T05:49:11.1306753Z Adding debian:ePKI_Root_Certification_Authority.pem
2019-10-30T05:49:11.1338819Z Adding debian:DigiCert_Assured_ID_Root_G3.pem
2019-10-30T05:49:11.1366771Z Adding debian:EE_Certification_Centre_Root_CA.pem
2019-10-30T05:49:11.1391977Z Adding debian:thawte_Primary_Root_CA_-_G3.pem
2019-10-30T05:49:11.1415024Z Adding debian:Secure_Global_CA.pem
2019-10-30T05:49:11.1452315Z Adding debian:SecureSign_RootCA11.pem
2019-10-30T05:49:11.1476115Z Adding debian:Certigna.pem
2019-10-30T05:49:11.1503121Z Adding debian:VeriSign_Universal_Root_Certification_Authority.pem
2019-10-30T05:49:11.1503121Z Adding debian:VeriSign_Universal_Root_Certification_Authority.pem
2019-10-30T05:49:11.1542632Z Adding debian:LuxTrust_Global_Root_2.pem
2019-10-30T05:49:11.1578791Z Adding debian:AffirmTrust_Premium.pem
2019-10-30T05:49:11.1598449Z Adding debian:GeoTrust_Global_CA.pem
2019-10-30T05:49:11.1626814Z Adding debian:CA_Disig_Root_R1.pem
2019-10-30T05:49:11.1645858Z Adding debian:Security_Communication_EV_RootCA1.pem
2019-10-30T05:49:11.1673136Z Adding debian:TWCA_Global_Root_CA.pem
2019-10-30T05:49:11.1717240Z Adding debian:Swisscom_Root_CA_2.pem
2019-10-30T05:49:11.1735680Z Adding debian:AffirmTrust_Premium_ECC.pem
2019-10-30T05:49:11.1757274Z Adding debian:SecureTrust_CA.pem
2019-10-30T05:49:11.1791882Z Adding debian:ACEDICOM_Root.pem
2019-10-30T05:49:11.1817899Z Adding debian:GeoTrust_Universal_CA.pem
2019-10-30T05:49:11.1851832Z Adding debian:Certplus_Root_CA_G1.pem
2019-10-30T05:49:11.1870899Z Adding debian:COMODO_ECC_Certification_Authority.pem
2019-10-30T05:49:11.1915137Z Adding debian:Certplus_Root_CA_G2.pem
2019-10-30T05:49:11.1991379Z Adding debian:Hellenic_Academic_and_Research_Institutions_ECC_RootCA_2015.pem
2019-10-30T05:49:11.2066601Z Adding debian:PSCProcert.pem
2019-10-30T05:49:11.2098186Z Adding debian:Microsec_e-Szigno_Root_CA_2009.pem
2019-10-30T05:49:11.2277611Z Adding debian:thawte_Primary_Root_CA.pem
2019-10-30T05:49:11.2347130Z Adding debian:AffirmTrust_Networking.pem
2019-10-30T05:49:11.2416539Z Adding debian:CA_Disig_Root_R2.pem
2019-10-30T05:49:11.2485876Z Adding debian:D-TRUST_Root_Class_3_CA_2_2009.pem
2019-10-30T05:49:11.2485876Z Adding debian:D-TRUST_Root_Class_3_CA_2_2009.pem
2019-10-30T05:49:11.2545255Z Adding debian:AddTrust_External_Root.pem
2019-10-30T05:49:11.2608581Z Adding debian:Comodo_Secure_Services_root.pem
2019-10-30T05:49:11.2690519Z Adding debian:GeoTrust_Universal_CA_2.pem
2019-10-30T05:49:11.2748841Z Adding debian:DigiCert_High_Assurance_EV_Root_CA.pem
2019-10-30T05:49:11.2786428Z Adding debian:thawte_Primary_Root_CA_-_G2.pem
2019-10-30T05:49:11.2821905Z Adding debian:GlobalSign_Root_CA.pem
2019-10-30T05:49:11.2867348Z Adding debian:Starfield_Class_2_CA.pem
2019-10-30T05:49:11.2907799Z Adding debian:Sonera_Class_2_Root_CA.pem
2019-10-30T05:49:11.2986157Z Adding debian:T-TeleSec_GlobalRoot_Class_3.pem
2019-10-30T05:49:11.3018942Z Adding debian:Actalis_Authentication_Root_CA.pem
2019-10-30T05:49:11.3089181Z Adding debian:Certum_Trusted_Network_CA.pem
2019-10-30T05:49:11.3150533Z Adding debian:Autoridad_de_Certificacion_Firmaprofesional_CIF_A62634068.pem
2019-10-30T05:49:11.3213556Z Adding debian:Camerfirma_Global_Chambersign_Root.pem
2019-10-30T05:49:11.3237412Z Adding debian:Network_Solutions_Certificate_Authority.pem
2019-10-30T05:49:11.3262984Z Adding debian:Starfield_Root_Certificate_Authority_-_G2.pem
2019-10-30T05:49:11.3299647Z Adding debian:Starfield_Services_Root_Certificate_Authority_-_G2.pem
2019-10-30T05:49:11.3341046Z Adding debian:SwissSign_Silver_CA_-_G2.pem
2019-10-30T05:49:11.3393574Z Adding debian:Go_Daddy_Root_Certificate_Authority_-_G2.pem
2019-10-30T05:49:11.3440510Z Adding debian:OpenTrust_Root_CA_G3.pem
2019-10-30T05:49:11.3510386Z Adding debian:DigiCert_Assured_ID_Root_CA.pem
2019-10-30T05:49:11.3558344Z Adding debian:NetLock_Arany_=Class_Gold=_Főtanúsítvány.pem
2019-10-30T05:49:11.3616406Z Adding debian:UTN_USERFirst_Hardware_Root_CA.pem
2019-10-30T05:49:11.3671983Z Adding debian:Entrust_Root_Certification_Authority_-_EC1.pem
2019-10-30T05:49:11.3713312Z Adding debian:Deutsche_Telekom_Root_CA_2.pem
2019-10-30T05:49:11.3792181Z Adding debian:DigiCert_Global_Root_CA.pem
2019-10-30T05:49:11.3879184Z Adding debian:DST_Root_CA_X3.pem
2019-10-30T05:49:11.3921129Z Adding debian:Certinomis_-_Root_CA.pem
2019-10-30T05:49:11.3921129Z Adding debian:Certinomis_-_Root_CA.pem
2019-10-30T05:49:11.3989106Z Adding debian:ACCVRAIZ1.pem
2019-10-30T05:49:11.4135808Z Adding debian:GeoTrust_Primary_Certification_Authority.pem
2019-10-30T05:49:11.4185369Z Adding debian:Staat_der_Nederlanden_EV_Root_CA.pem
2019-10-30T05:49:11.4185369Z Adding debian:Staat_der_Nederlanden_EV_Root_CA.pem
2019-10-30T05:49:11.4216554Z Adding debian:D-TRUST_Root_Class_3_CA_2_EV_2009.pem
2019-10-30T05:49:11.4279006Z Adding debian:GlobalSign_Root_CA_-_R3.pem
2019-10-30T05:49:11.4313073Z Adding debian:TURKTRUST_Certificate_Services_Provider_Root_2007.pem
2019-10-30T05:49:11.4346836Z Adding debian:GeoTrust_Global_CA_2.pem
2019-10-30T05:49:11.4400595Z Adding debian:Certum_Trusted_Network_CA_2.pem
2019-10-30T05:49:11.4432090Z Adding debian:XRamp_Global_CA_Root.pem
2019-10-30T05:49:11.4476832Z Adding debian:DigiCert_Global_Root_G2.pem
2019-10-30T05:49:11.4511332Z Adding debian:Trustis_FPS_Root_CA.pem
2019-10-30T05:49:11.5105258Z Setting up libgcc1:i386 (1:6.0.1-0ubuntu1) ...
2019-10-30T05:49:11.5616964Z Setting up libc6:i386 (2.23-0ubuntu11) ...
2019-10-30T05:49:11.6848021Z debconf: unable to initialize frontend: Dialog
2019-10-30T05:49:11.6848147Z debconf: (TERM is not set, so the dialog frontend is not usable.)
---
2019-10-30T05:49:13.5908502Z Running hooks in /etc/ca-certificates/update.d...
2019-10-30T05:49:13.5956927Z 
2019-10-30T05:49:13.9742501Z done.
2019-10-30T05:49:13.9747729Z done.
2019-10-30T05:49:14.0112982Z Setting up openjdk-9-jre-headless:amd64 (9~b114-0ubuntu1) ...
2019-10-30T05:49:14.3869724Z update-alternatives: using /usr/lib/jvm/java-9-openjdk-amd64/bin/rmid to provide /usr/bin/rmid (rmid) in auto mode
2019-10-30T05:49:14.4037396Z update-alternatives: using /usr/lib/jvm/java-9-openjdk-amd64/bin/java to provide /usr/bin/java (java) in auto mode
2019-10-30T05:49:14.4157078Z update-alternatives: using /usr/lib/jvm/java-9-openjdk-amd64/bin/keytool to provide /usr/bin/keytool (keytool) in auto mode
2019-10-30T05:49:14.4249966Z update-alternatives: using /usr/lib/jvm/java-9-openjdk-amd64/bin/jjs to provide /usr/bin/jjs (jjs) in auto mode
2019-10-30T05:49:14.4347156Z update-alternatives: using /usr/lib/jvm/java-9-openjdk-amd64/bin/pack200 to provide /usr/bin/pack200 (pack200) in auto mode
2019-10-30T05:49:14.4457946Z update-alternatives: using /usr/lib/jvm/java-9-openjdk-amd64/bin/rmiregistry to provide /usr/bin/rmiregistry (rmiregistry) in auto mode
2019-10-30T05:49:14.4567778Z update-alternatives: using /usr/lib/jvm/java-9-openjdk-amd64/bin/unpack200 to provide /usr/bin/unpack200 (unpack200) in auto mode
2019-10-30T05:49:14.4667907Z update-alternatives: using /usr/lib/jvm/java-9-openjdk-amd64/bin/orbd to provide /usr/bin/orbd (orbd) in auto mode
2019-10-30T05:49:14.4777607Z update-alternatives: using /usr/lib/jvm/java-9-openjdk-amd64/bin/servertool to provide /usr/bin/servertool (servertool) in auto mode
2019-10-30T05:49:14.4888301Z update-alternatives: using /usr/lib/jvm/java-9-openjdk-amd64/bin/tnameserv to provide /usr/bin/tnameserv (tnameserv) in auto mode
2019-10-30T05:49:14.4989491Z update-alternatives: using /usr/lib/jvm/java-9-openjdk-amd64/lib/jexec to provide /usr/bin/jexec (jexec) in auto mode
2019-10-30T05:49:14.8229882Z Processing triggers for systemd (229-4ubuntu21.22) ...
2019-10-30T05:49:33.0244912Z Removing intermediate container aa153b6fb2c2
2019-10-30T05:49:33.0246271Z  ---> 27dbbc187eba
2019-10-30T05:49:33.0247060Z Step 7/20 : COPY scripts/android-sdk.sh /scripts/
---
2019-10-30T05:49:39.4935760Z   5  165M    5 8793k    0     0  8796k      0  0:00:19 --:--:--  0:00:19 8793k
2019-10-30T05:49:39.4935966Z  17  165M   17 28.6M    0     0  19.7M      0  0:00:08  0:00:01  0:00:07 19.7M
2019-10-30T05:49:39.5026714Z curl: (56) GnuTLS recv error (-24): Decryption has failed.
2019-10-30T05:49:39.5091857Z Traceback (most recent call last):
2019-10-30T05:49:39.5092507Z   File "/scripts/android-sdk-manager.py", line 192, in <module>
2019-10-30T05:49:39.5092773Z     cli()
2019-10-30T05:49:39.5093205Z   File "/scripts/android-sdk-manager.py", line 189, in cli
2019-10-30T05:49:39.5093446Z     args.func(args)
2019-10-30T05:49:39.5094036Z   File "/scripts/android-sdk-manager.py", line 150, in cli_install
2019-10-30T05:49:39.5094243Z     downloaded = package.download(url)
2019-10-30T05:49:39.5094759Z   File "/scripts/android-sdk-manager.py", line 51, in download
2019-10-30T05:49:39.5095158Z     subprocess.run(["curl", "-o", file, url], check=True)
2019-10-30T05:49:39.5095375Z   File "/usr/lib/python3.5/subprocess.py", line 708, in run
2019-10-30T05:49:39.5095534Z     output=stdout, stderr=stderr)
2019-10-30T05:49:39.5096063Z subprocess.CalledProcessError: Command '['curl', '-o', '/tmp/tmp8ponxu91', 'https://rust-lang-ci-mirrors.s3-us-west-1.amazonaws.com/rustc/android/emulator-linux-5264690.zip']' returned non-zero exit status 56
2019-10-30T05:49:40.4825988Z The command '/bin/sh -c /scripts/android-sdk.sh' returned a non-zero code: 1
2019-10-30T05:49:42.5882778Z Sending build context to Docker daemon  524.3kB
2019-10-30T05:49:42.5883553Z 
2019-10-30T05:49:42.6107074Z Step 1/20 : FROM ubuntu:16.04
2019-10-30T05:49:42.6121623Z  ---> b9409899fe86
---
2019-10-30T05:51:35.7430525Z  44  124M   44 55.5M    0     0  6410k      0  0:00:19  0:00:08  0:00:11 6703k
2019-10-30T05:51:35.7430677Z  47  124M   47 59.0M    0     0  6296k      0  0:00:20  0:00:09  0:00:11 6297k
2019-10-30T05:51:35.7497558Z curl: (56) GnuTLS recv error (-24): Decryption has failed.
2019-10-30T05:51:35.9995545Z Traceback (most recent call last):
2019-10-30T05:51:35.9997078Z   File "/scripts/android-sdk-manager.py", line 192, in <module>
2019-10-30T05:51:35.9997614Z     cli()
2019-10-30T05:51:35.9998286Z   File "/scripts/android-sdk-manager.py", line 189, in cli
2019-10-30T05:51:35.9998591Z     args.func(args)
2019-10-30T05:51:35.9999130Z   File "/scripts/android-sdk-manager.py", line 150, in cli_install
2019-10-30T05:51:35.9999584Z     downloaded = package.download(url)
2019-10-30T05:51:36.0000047Z   File "/scripts/android-sdk-manager.py", line 51, in download
2019-10-30T05:51:36.0000539Z     subprocess.run(["curl", "-o", file, url], check=True)
2019-10-30T05:51:36.0000820Z   File "/usr/lib/python3.5/subprocess.py", line 708, in run
2019-10-30T05:51:36.0001059Z     output=stdout, stderr=stderr)
2019-10-30T05:51:36.0001844Z subprocess.CalledProcessError: Command '['curl', '-o', '/tmp/tmpcho0moqw', 'https://rust-lang-ci-mirrors.s3-us-west-1.amazonaws.com/rustc/android/sys-img/android/armeabi-v7a-18_r05.zip']' returned non-zero exit status 56
2019-10-30T05:51:46.1361613Z The command '/bin/sh -c /scripts/android-sdk.sh' returned a non-zero code: 1
2019-10-30T05:51:49.2453013Z Sending build context to Docker daemon  524.3kB
2019-10-30T05:51:49.2454171Z 
2019-10-30T05:51:49.2625714Z Step 1/20 : FROM ubuntu:16.04
2019-10-30T05:51:49.2629800Z  ---> b9409899fe86
---
2019-10-30T05:52:31.7977561Z   3  124M    3 4605k    0     0  3616k      0  0:00:35  0:00:01  0:00:34 3615k
2019-10-30T05:52:31.7977748Z   7  124M    7  9.8M    0     0  4885k      0  0:00:26  0:00:02  0:00:24 4887k
2019-10-30T05:52:31.8052750Z curl: (56) GnuTLS recv error (-24): Decryption has failed.
2019-10-30T05:52:31.8130443Z Traceback (most recent call last):
2019-10-30T05:52:31.8130729Z   File "/scripts/android-sdk-manager.py", line 192, in <module>
2019-10-30T05:52:31.8130848Z     cli()
2019-10-30T05:52:31.8131057Z   File "/scripts/android-sdk-manager.py", line 189, in cli
2019-10-30T05:52:31.8131149Z     args.func(args)
2019-10-30T05:52:31.8131367Z   File "/scripts/android-sdk-manager.py", line 150, in cli_install
2019-10-30T05:52:31.8131452Z     downloaded = package.download(url)
2019-10-30T05:52:31.8131678Z   File "/scripts/android-sdk-manager.py", line 51, in download
2019-10-30T05:52:31.8131889Z     subprocess.run(["curl", "-o", file, url], check=True)
2019-10-30T05:52:31.8131975Z   File "/usr/lib/python3.5/subprocess.py", line 708, in run
2019-10-30T05:52:31.8132038Z     output=stdout, stderr=stderr)
2019-10-30T05:52:31.8132430Z subprocess.CalledProcessError: Command '['curl', '-o', '/tmp/tmp5k2zzilo', 'https://rust-lang-ci-mirrors.s3-us-west-1.amazonaws.com/rustc/android/sys-img/android/armeabi-v7a-18_r05.zip']' returned non-zero exit status 56
2019-10-30T05:52:43.5846229Z The command '/bin/sh -c /scripts/android-sdk.sh' returned a non-zero code: 1
2019-10-30T05:52:47.6685687Z Sending build context to Docker daemon  524.3kB
2019-10-30T05:52:47.6687095Z 
2019-10-30T05:52:47.6865310Z Step 1/20 : FROM ubuntu:16.04
2019-10-30T05:52:47.6868950Z  ---> b9409899fe86
---
2019-10-30T05:54:54.5349350Z  54  124M   54 68.1M    0     0  6617k      0  0:00:19  0:00:10  0:00:09 2790k
2019-10-30T05:54:54.5349543Z  55  124M   55 68.9M    0     0  6365k      0  0:00:20  0:00:11  0:00:09 2682k
2019-10-30T05:54:54.5420250Z curl: (56) GnuTLS recv error (-24): Decryption has failed.
2019-10-30T05:54:54.9629538Z Traceback (most recent call last):
2019-10-30T05:54:54.9630008Z   File "/scripts/android-sdk-manager.py", line 192, in <module>
2019-10-30T05:54:54.9630124Z     cli()
2019-10-30T05:54:54.9630390Z   File "/scripts/android-sdk-manager.py", line 189, in cli
2019-10-30T05:54:54.9630493Z     args.func(args)
2019-10-30T05:54:54.9630768Z   File "/scripts/android-sdk-manager.py", line 150, in cli_install
2019-10-30T05:54:54.9630874Z     downloaded = package.download(url)
2019-10-30T05:54:54.9631171Z   File "/scripts/android-sdk-manager.py", line 51, in download
2019-10-30T05:54:54.9631681Z     subprocess.run(["curl", "-o", file, url], check=True)
2019-10-30T05:54:54.9631772Z   File "/usr/lib/python3.5/subprocess.py", line 708, in run
2019-10-30T05:54:54.9631834Z     output=stdout, stderr=stderr)
2019-10-30T05:54:54.9632383Z subprocess.CalledProcessError: Command '['curl', '-o', '/tmp/tmpvrz8_r7h', 'https://rust-lang-ci-mirrors.s3-us-west-1.amazonaws.com/rustc/android/sys-img/android/armeabi-v7a-18_r05.zip']' returned non-zero exit status 56
2019-10-30T05:55:00.8916785Z The command '/bin/sh -c /scripts/android-sdk.sh' returned a non-zero code: 1
2019-10-30T05:55:00.8957017Z 
2019-10-30T05:55:00.8957017Z 
2019-10-30T05:55:00.9052305Z ##[error]Bash exited with code '1'.
2019-10-30T05:55:00.9084060Z ##[section]Starting: Upload CPU usage statistics
2019-10-30T05:55:00.9090491Z ==============================================================================
2019-10-30T05:55:00.9090595Z Task         : Bash
2019-10-30T05:55:00.9090690Z Description  : Run a Bash script on macOS, Linux, or Windows
