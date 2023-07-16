plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0160fb30
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
$ export IMAGE=dist-x86_64-linux
$ export DEPLOY=1
$ bash -c 'echo $BASH_VERSION'
4.3.11(1)-release
Debug build initiated by kennytm
Setting up debug tools.
Preparing debug sessions.
Use the following SSH command to access the interactive debugging environment:
ssh LgBTRvkX4AqAR5jqQFYADQwUv@ny2.tmate.io
Output from the interactive session will be shown below:
travis@travis-job-4b636612-9db1-48e7-99b5-438cced91464:~/build/rust-lang/rust$ lsls
appveyor.yml  CODE_OF_CONDUCT.md  config.toml.example  configure  CONTRIBUTING.md  COPYRIGHT  LICENSE-APACHE  LICENSE-MIT  README.md  RELEASES.md  src x.py
travis@travis-job-4b636612-9db1-48e7-99b5-438cced91464:~/build/rust-lang/rust$ cd /checkout
bash: cd: /checkout: No such file or directory
travis@travis-job-4b636612-9db1-48e7-99b5-438cced91464:~/build/rust-lang/rust$ cd /
travis@travis-job-4b636612-9db1-48e7-99b5-438cced91464:/$ ls
bin  boot  dev etc  home  initrd.img  initrd.img.old  lib  lib64  lost+found  media  mnt  opt proc  root  run  sbin  srv  sys  tmp  usr  var vmlinuz  vmlinuz.old
travis@travis-job-4b636612-9db1-48e7-99b5-438cced91464:/$ cat /var/log/syslog
cat: /var/log/syslog: Permission denied
travis@travis-job-4b636612-9db1-48e7-99b5-438cced91464:/$ cat /var/log/syslog[1@ 
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 dbus[1244]: [system] Activating service name='org.freedesktop.systemd1' (using servicehelper)
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 dbus[1244]: [system] Successfully activated service 'org.freedesktop.systemd1'
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.280443] init: tty4 main process (1593) killed by TERM signal
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.280616] init: tty5 main process (1595) killed by TERM signal
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.280740] init: tty2 main process (1599) killed by TERM signal
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.280869] init: tty3 main process (1600) killed by TERM signal
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.281009] init: tty6 main process (1602) killed by TERM signal
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.281251] init: cron main process (1637) killed by TERM signal
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.281503] init: tty1 main process (1828) killed by TERM signal
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.281658] init: ttyS0 main process (1837) killed by TERM signal
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.281781] init: irqbalance main process (23872) killed by TERM signal
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.282014] init: plymouth-upstart-bridge main process (1221) terminated with status 1
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.282020] init: plymouth-upstart-bridge main process ended, respawning
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.329446] init: plymouth-upstart-bridge main process (1253) terminated with status 1
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.329456] init: plymouth-upstart-bridge main process ended, respawning
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.346769] init: plymouth-upstart-bridge main process (1256) terminated with status 1
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.346779] init: plymouth-upstart-bridge main process ended, respawning
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.354732] init: plymouth-upstart-bridge main process (1259) terminated with status 1
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.354741] init: plymouth-upstart-bridge main process ended, respawning
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.363072] init: plymouth-upstart-bridge main process (1262) terminated with status 1
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.363083] init: plymouth-upstart-bridge main process ended, respawning
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.373178] init: plymouth-upstart-bridge main process (1266) terminated with status 1
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.373187] init: plymouth-upstart-bridge main process ended, respawning
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.380984] init: plymouth-upstart-bridge main process (1274) terminated with status 1
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.380993] init: plymouth-upstart-bridge main process ended, respawning
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.384956] init: plymouth-upstart-bridge main process (1276) terminated with status 1
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.384965] init: plymouth-upstart-bridge main process ended, respawning
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.390662] init: plymouth-upstart-bridge main process (1277) terminated with status 1
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.390671] init: plymouth-upstart-bridge main process ended, respawning
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.395025] init: plymouth-upstart-bridge main process (1281) terminated with status 1
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.395035] init: plymouth-upstart-bridge main process ended, respawning
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.401049] init: plymouth-upstart-bridge main process (1283) terminated with status 1
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.401059] init: plymouth-upstart-bridge respawning too fast, stopped
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 shutdown-script: INFO Starting shutdown scripts.
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 shutdown-script: INFO No shutdown scripts found in metadata.
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 shutdown-script: INFO Finished running shutdown scripts.
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 kernel: [ 1108.433693] init: wait-for-state (google-shutdown-scripts-block-rsysloggoogle-shutdown-scripts) main process (1265) killed by TERM signal
Dec  5 19:49:36 packer-5a26f3f3-939e-118e-ee3f-93dbd7aee985 rsyslogd: [origin software="rsyslogd" swVersion="7.4.4" x-pid="1291" x-info="http://www.rsyslog.com"] exiting on signal 15.
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 rsyslogd: [origin software="rsyslogd" swVersion="7.4.4" x-pid="1198" x-info="http://www.rsyslog.com"] start
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 rsyslogd-2307: warning: ~ action is deprecated, consider using the 'stop' statement instead [try http://www.rsyslog.com/e/2307 ]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 rsyslogd: rsyslogd's groupid changed to 104
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 rsyslogd: rsyslogd's userid changed to 101
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Initializing cgroup subsys cpuset
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Initializing cgroup subsys cpu
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Initializing cgroup subsys cpuacct
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Linux version 4.4.0-101-generic (buildd@lgw01-amd64-031) (gcc version 4.8.4 (Ubuntu 4.8.4-2ubuntu1~14.04.3) ) #124~14.04.1-Ubuntu SMP Fri Nov 10 19:05:36 UTC 2017 (Ubuntu 4.4.0-101.124~14.04.1-generic 4.4.95)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] KERNEL supported cpus:
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   Intel GenuineIntel
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   AMD AuthenticAMD
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   Centaur CentaurHauls
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] x86/fpu: xstate_offset[2]:  576, xstate_sizes[2]:  256
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] x86/fpu: Supporting XSAVE feature 0x01: 'x87 floating point registers'
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] x86/fpu: Supporting XSAVE feature 0x02: 'SSE registers'
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] x86/fpu: Supporting XSAVE feature 0x04: 'AVX registers'
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] x86/fpu: Enabled xstate features 0x7, context size is 832 bytes, using 'standard' format.
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] x86/fpu: Using 'eager' FPU context switches.
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] e820: BIOS-provided physical RAM map:
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] BIOS-e820: [mem 0x0000000000000000-0x000000000009fbff] usable
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] BIOS-e820: [mem 0x000000000009fc00-0x000000000009ffff] reserved
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] BIOS-e820: [mem 0x00000000000f0000-0x00000000000fffff] reserved
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] BIOS-e820: [mem 0x0000000000100000-0x00000000bfffcfff] usable
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] BIOS-e820: [mem 0x00000000bfffd000-0x00000000bfffffff] reserved
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] BIOS-e820: [mem 0x00000000fffbc000-0x00000000ffffffff] reserved
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] BIOS-e820: [mem 0x0000000100000000-0x00000003ffffffff] usable
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] NX (Execute Disable) protection: active
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] SMBIOS 2.4 present.
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] DMI: Google Google Compute Engine/Google Compute Engine, BIOS Google 01/01/2011
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Hypervisor detected: KVM
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] e820: update [mem 0x00000000-0x00000fff] usable ==> reserved
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] e820: remove [mem 0x000a0000-0x000fffff] usable
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] e820: last_pfn = 0x400000 max_arch_pfn = 0x400000000
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] MTRR default type: write-back
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] MTRR fixed ranges enabled:
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   00000-9FFFF write-back
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   A0000-BFFFF uncachable
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   C0000-FFFFF write-protect
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] MTRR variable ranges enabled:
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   0 base 0000C0000000 mask 3FFFC0000000 uncachable
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   1 disabled
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   2 disabled
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   3 disabled
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   4 disabled
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   5 disabled
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   6 disabled
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   7 disabled
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] e820: last_pfn = 0xbfffd max_arch_pfn = 0x400000000
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] found SMP MP-table at [mem 0x000f2c80-0x000f2c8f] mapped at [ffff8800000f2c80]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Scanning 1 areas for low memory corruption
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Using GB pages for direct mapping
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: RSDP 0x00000000000F29F0 000014 (v00 Google)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFFDAA0 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFFF00 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFFDAE0 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFFEC0 000040
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFFEC0 000040
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFFF4C0 0009F2 (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: APIC 0x00000000BFFFF3C0 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: WAET 0x00000000BFFFF390 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFFF2A0 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] kvm-clock: using sched offset of 1685872382 cycles
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Zone ranges:
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   Device   empty
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Movable zone start for each node
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Early memory node ranges
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfffcfff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] On node 0 totalpages: 3932059
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   DMA zone: 21 pages reserved
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   DMA32 zone: 782333 pages, LIFO batch:31
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: IRQ5 used by override.
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: IRQ9 used by override.
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: IRQ10 used by override.
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] ACPI: IRQ11 used by override.
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfffd000-0xbfffffff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870598
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Policy zone: Normal
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Memory: 15375532K/15728236K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Hierarchical RCU implementation.
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] Console: colour VGA+ 80x25
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] console [ttyS0] enabled
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.347722] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.349226] pid_max: default: 32768 minimum: 301
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.350078] ACPI: Core revision 20150930
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.351820] ACPI: 2 ACPI AML tables successfully acquired and loaded
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.353616] Security Framework initialized
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.354354] Yama: becoming mindful.
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.355310] AppArmor: AppArmor disabled by boot time parameter
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.358132] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.367383] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.371638] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.373516] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.374960] Initializing cgroup subsys io
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.375563] Initializing cgroup subsys memory
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.376281] Initializing cgroup subsys devices
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.377167] Initializing cgroup subsys freezer
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.378145] Initializing cgroup subsys net_cls
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.379104] Initializing cgroup subsys perf_event
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.380125] Initializing cgroup subsys net_prio
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.380805] Initializing cgroup subsys hugetlb
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.381536] Initializing cgroup subsys pids
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.382439] CPU: Physical Processor ID: 0
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.383222] CPU: Processor Core ID: 0
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.383788] mce: CPU supports 32 MCE banks
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.384542] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.385380] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.389476] Freeing SMP alternatives memory: 32K
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.402778] ftrace: allocating 32185 entries in 126 pages
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.472580] smpboot: APIC(0) Converting physical 0 to logical package 0
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.473903] smpboot: Max logical packages: 2
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.475254] x2apic enabled
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.477155] Switched APIC routing to physical x2apic.
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.481262] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.586802] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.589541] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.592047] x86: Booting SMP configuration:
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.592753] .... node  #0, CPUs:      #1
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.594084] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.597764]  #2
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.598255] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.601697]  #3
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.602271] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.605807] x86: Booted up 1 node, 4 CPUs
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.606797] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.609735] devtmpfs: initialized
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.613383] evm: security.selinux
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.613939] evm: security.SMACK64
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.614431] evm: security.SMACK64EXEC
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.615147] evm: security.SMACK64TRANSMUTE
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.615867] evm: security.SMACK64MMAP
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.616435] evm: security.ima
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.616931] evm: security.capability
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.618529] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.620243] futex hash table entries: 1024 (order: 4, 65536 bytes)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.621489] pinctrl core: initialized pinctrl subsystem
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.622613] RTC time:  8:21:02, date: 11/21/18
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.624418] NET: Registered protocol family 16
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.634908] cpuidle: using governor ladder
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.646836] cpuidle: using governor menu
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.647703] PCCT header not found.
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.648333] ACPI: bus type PCI registered
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.649155] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.650312] PCI: Using configuration type 1 for base access
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.663896] ACPI: Added _OSI(Module Device)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.664921] ACPI: Added _OSI(Processor Device)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.665814] ACPI: Added _OSI(3.0 _SCP Extensions)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.666900] ACPI: Added _OSI(Processor Aggregator Device)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.668780] ACPI: Executed 2 blocks of module-level executable AML code
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.671140] ACPI: Interpreter enabled
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.671961] ACPI: (supports S0 S3 S4 S5)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.672819] ACPI: Using IOAPIC for interrupt routing
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.673669] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.677394] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.678408] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.679616] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.680992] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.683492] PCI host bridge to bus 0000:00
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.684124] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.685290] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.686309] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.687577] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.688983] pci_bus 0000:00: root bus resource [bus 00-ff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.690020] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.690514] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.704610] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.718497] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.720374] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.725097] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.729621] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.743742] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.749105] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.753296] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.766200] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.768870] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.771498] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.774306] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.777438] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.779130] ACPI: Enabled 16 GPEs in block 00 to 0F
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.780225] vgaarb: loaded
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.780974] SCSI subsystem initialized
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.781742] libata version 3.00 loaded.
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.781766] ACPI: bus type USB registered
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.782850] usbcore: registered new interface driver usbfs
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.783843] usbcore: registered new interface driver hub
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.784684] usbcore: registered new device driver usb
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.786134] PCI: Using ACPI for IRQ routing
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.786815] PCI: pci_cache_line_size set to 64 bytes
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.786924] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.786926] e820: reserve RAM buffer [mem 0xbfffd000-0xbfffffff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.787058] NetLabel: Initializing
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.787675] NetLabel:  domain hash size = 128
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.788414] NetLabel:  protocols = UNLABELED CIPSOv4
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.789294] NetLabel:  unlabeled traffic allowed by default
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.790285] amd_nb: Cannot enumerate AMD northbridges
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.791187] clocksource: Switched to clocksource kvm-clock
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.799282] pnp: PnP ACPI init
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.800001] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.800078] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.800124] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.800186] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.800231] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.800273] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.800316] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.800469] pnp: PnP ACPI: found 7 devices
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.809508] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.810942] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.810944] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.810946] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.810947] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.810982] NET: Registered protocol family 2
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.811854] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.813977] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.815435] TCP: Hash tables configured (established 131072 bind 65536)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.816503] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.817757] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.819000] NET: Registered protocol family 1
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.819805] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.820752] PCI: CLS 0 bytes, default 64
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    0.820807] Unpacking initramfs...
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.857473] Freeing initrd memory: 21432K
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.858422] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.859647] software IO TLB [mem 0xbbffd000-0xbfffd000] (64MB) mapped at [ffff8800bbffd000-ffff8800bfffcfff]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.861566] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.863321] hw unit of domain pp0-core 2^-0 Joules
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.864152] hw unit of domain package 2^-0 Joules
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.864839] hw unit of domain dram 2^-0 Joules
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.865608] Scanning for low memory corruption every 60 seconds
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.867175] audit: initializing netlink subsys (disabled)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.868089] audit: type=2000 audit(1542788464.232:1): initialized
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.869313] Initialise system trusted keyring
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.870340] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.871388] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.873914] zbud: loaded
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.874833] VFS: Disk quotas dquot_6.6.0
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.875600] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.877095] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.878593] fuse init (API version 7.23)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.879559] Key type big_key registered
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.880433] Allocating IMA MOK and blacklist keyrings.
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.883090] Key type asymmetric registered
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.884186] Asymmetric key parser 'x509' registered
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.885306] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.886982] io scheduler noop registered
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.887857] io scheduler deadline registered (default)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.888924] io scheduler cfq registered
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.890112] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.891495] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.893257] intel_idle: does not run on family 6 model 45
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.893339] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.894835] ACPI: Power Button [PWRF]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.895566] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.896841] ACPI: Sleep Button [SLPF]
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.897978] GHES: HEST is not enabled!
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.900734] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.901967] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.906611] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.907904] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.912390] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.935453] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.958904] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    2.982200] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.005658] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.009438] Linux agpgart interface v0.103
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.012808] loop: module loaded
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.013920] libphy: Fixed MDIO Bus: probed
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.015118] tun: Universal TUN/TAP device driver, 1.6
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.016283] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.046016] PPP generic driver version 2.4.2
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.047076] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.048779] ehci-pci: EHCI PCI platform driver
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.049810] ehci-platform: EHCI generic platform driver
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.050992] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.052469] ohci-pci: OHCI PCI platform driver
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.053495] ohci-platform: OHCI generic platform driver
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.054595] uhci_hcd: USB Universal Host Controller Interface driver
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.055990] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.058603] i8042: Warning: Keylock active
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.060628] serio: i8042 KBD port at 0x60,0x64 irq 1
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.061738] serio: i8042 AUX port at 0x60,0x64 irq 12
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.063141] mousedev: PS/2 mouse device common for all mice
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.064700] rtc_cmos 00:00: RTC can wake from S4
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.066074] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.067591] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.069252] i2c /dev entries driver
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.070144] device-mapper: uevent: version 1.0.3
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.071189] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.073160] ledtrig-cpu: registered to indicate activity on CPUs
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.075816] NET: Registered protocol family 10
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.077112] NET: Registered protocol family 17
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.078785] Key type dns_resolver registered
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.080755] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.082095] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.084188] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.085568] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.087458] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.091427] registered taskstats version 1
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.092793] Loading compiled-in X.509 certificates
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.095120] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.098612] zswap: loaded using pool lzo/zbud
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.102179] Key type trusted registered
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.106383] Key type encrypted registered
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.107916] ima: No TPM chip found, activating TPM-bypass!
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.109951] evm: HMAC attrs: 0x1
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.111571]   Magic number: 10:279:372
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.113166] rtc_cmos 00:00: setting system clock to 2018-11-21 08:21:04 UTC (1542788464)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.116104] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.117393] EDD information not available.
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.118340] PM: Hibernation image not present or could not be loaded.
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.119748] Freeing unused kernel memory: 1496K
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.120423] Write protecting the kernel read-only data: 14336k
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.122379] Freeing unused kernel memory: 1956K
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.123569] Freeing unused kernel memory: 92K
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.138150] systemd-udevd[119]: starting version 204
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.178885] scsi host0: Virtio SCSI HBA
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.181259] AVX version of gcm_enc/dec engaged.
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.182735] AES CTR mode by8 optimization enabled
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.186349] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.226561] sd 0:0:1:0: [sda] 146800640 512-byte logical blocks: (75.2 GB/70.0 GiB)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.226618] sd 0:0:1:0: Attached scsi generic sg0 type 0
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.229193] sd 0:0:1:0: [sda] 4096-byte physical blocks
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.230495] sd 0:0:1:0: [sda] Write Protect is off
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.231555] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.231680] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.235112]  sda: sda1
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.237045] sd 0:0:1:0: [sda] Attached SCSI disk
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.267554] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.863405] tsc: Refined TSC clocksource calibration: 2600.001 MHz
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    3.864548] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3ce1c4c, max_idle_ns: 440795206275 ns
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    4.104473] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    6.187505] floppy0: no floppy controllers found
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    7.355209] raid6: sse2x1   gen()  8910 MB/s
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    7.423211] raid6: sse2x1   xor()  6633 MB/s
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    7.491212] raid6: sse2x2   gen() 10921 MB/s
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    7.559211] raid6: sse2x2   xor()  7229 MB/s
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    7.627210] raid6: sse2x4   gen() 12710 MB/s
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    7.695209] raid6: sse2x4   xor()  8853 MB/s
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    7.696288] raid6: using algorithm sse2x4 gen() 12710 MB/s
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    7.697175] raid6: .... xor() 8853 MB/s, rmw enabled
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    7.697890] raid6: using ssse3x2 recovery algorithm
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    7.700386] xor: automatically using best checksumming function:
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    7.739205]    avx       : 27534.000 MB/sec
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    7.754673] Btrfs loaded
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    7.792212] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    7.793459] EXT4-fs (sda1): write access will be enabled during recovery
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    7.872470] EXT4-fs (sda1): orphan cleanup on readonly fs
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    7.877732] EXT4-fs (sda1): 6 orphan inodes deleted
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    7.878778] EXT4-fs (sda1): recovery complete
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    7.883386] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    8.077852] random: init: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    8.171262] random: mountall: uninitialized urandom read (12 bytes read, 31 bits of entropy available)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    8.219144] EXT4-fs (sda1): re-mounted. Opts: (null)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    8.389774] random: cloud-init: uninitialized urandom read (32 bytes read, 38 bits of entropy available)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    8.885485] random: cloud-init: uninitialized urandom read (32 bytes read, 47 bits of entropy available)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    9.016297] systemd-udevd[702]: starting version 204
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    9.105606] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    9.123427] intel_rapl: no valid rapl domains found in package 0
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    9.168683] intel_rapl: no valid rapl domains found in package 0
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    9.207515] intel_rapl: no valid rapl domains found in package 0
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    9.211247] ppdev: user-space parallel port driver
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    9.305041] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    9.351987] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    9.420874] random: cloud-init: uninitialized urandom read (32 bytes read, 60 bits of entropy available)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    9.584966] random: cloud-init: uninitialized urandom read (32 bytes read, 60 bits of entropy available)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    9.863434] random: mktemp: uninitialized urandom read (12 bytes read, 63 bits of entropy available)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    9.931721] random: mktemp: uninitialized urandom read (6 bytes read, 63 bits of entropy available)
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [    9.999528] EXT4-fs (sda1): resizing filesystem from 3931904 to 18349824 blocks
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [   10.062630] EXT4-fs (sda1): resized filesystem to 18349824
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 kernel: [   10.396320] init: failsafe main process (1094) killed by TERM signal
Nov 21 08:21:11 travis-job-4b636612-9db1-48e7-99b5-438cced91464 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Nov 21 08:21:12 travis-job-4b636612-9db1-48e7-99b5-438cced91464 instance-setup: INFO Running set_multiqueue.
Nov 21 08:21:12 travis-job-4b636612-9db1-48e7-99b5-438cced91464 instance-setup: INFO Set channels for eth0 to 4.
Nov 21 08:21:12 travis-job-4b636612-9db1-48e7-99b5-438cced91464 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Nov 21 08:21:12 travis-job-4b636612-9db1-48e7-99b5-438cced91464 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Nov 21 08:21:12 travis-job-4b636612-9db1-48e7-99b5-438cced91464 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Nov 21 08:21:12 travis-job-4b636612-9db1-48e7-99b5-438cced91464 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Nov 21 08:21:12 travis-job-4b636612-9db1-48e7-99b5-438cced91464 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
