plain
Requested labels: ubuntu-20.04-16core-64gb
Job defined at: rust-lang/rust/.github/workflows/ci.yml@refs/pull/109332/merge
Waiting for a runner to pick up this job...
Job is about to start running on the runner: ubuntu-20.04-16core-64gb_e5be460c9b40 (organization)
Current runner version: '2.303.0'
Runner name: 'ubuntu-20.04-16core-64gb_e5be460c9b40'
Runner group name: 'Default Larger Runners'
Machine name: 'runner'
Ubuntu
20.04.5
LTS
##[endgroup]
---
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
processor : 0
vendor_id : AuthenticAMD
cpu family : 25
model  : 1
model name : AMD EPYC 7763 64-Core Processor
microcode : 0xffffffff
cpu MHz  : 2445.430
cache size : 512 KB
physical id : 0
---
fpu  : yes
fpu_exception : yes
cpuid level : 13
wp  : yes
flags  : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext invpcid_single vmmcall fsgsbase bmi1 avx2 smep bmi2 invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid
bugs  : sysret_ss_attrs null_seg spectre_v1 spectre_v2 spec_store_bypass
bogomips : 4890.86
TLB size : 2560 4K pages
cache_alignment : 64
address sizes : 48 bits physical, 48 bits virtual
power management:


processor : 1
vendor_id : AuthenticAMD
cpu family : 25
model  : 1
model name : AMD EPYC 7763 64-Core Processor
microcode : 0xffffffff
cpu MHz  : 2445.430
cache size : 512 KB
physical id : 0
---
fpu  : yes
fpu_exception : yes
cpuid level : 13
wp  : yes
flags  : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext invpcid_single vmmcall fsgsbase bmi1 avx2 smep bmi2 invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid
bugs  : sysret_ss_attrs null_seg spectre_v1 spectre_v2 spec_store_bypass
bogomips : 4890.86
TLB size : 2560 4K pages
cache_alignment : 64
address sizes : 48 bits physical, 48 bits virtual
power management:


processor : 2
vendor_id : AuthenticAMD
cpu family : 25
model  : 1
model name : AMD EPYC 7763 64-Core Processor
microcode : 0xffffffff
cpu MHz  : 2445.430
cache size : 512 KB
physical id : 0
---
fpu  : yes
fpu_exception : yes
cpuid level : 13
wp  : yes
flags  : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext invpcid_single vmmcall fsgsbase bmi1 avx2 smep bmi2 invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid
bugs  : sysret_ss_attrs null_seg spectre_v1 spectre_v2 spec_store_bypass
bogomips : 4890.86
TLB size : 2560 4K pages
cache_alignment : 64
address sizes : 48 bits physical, 48 bits virtual
power management:


processor : 3
vendor_id : AuthenticAMD
cpu family : 25
model  : 1
model name : AMD EPYC 7763 64-Core Processor
microcode : 0xffffffff
cpu MHz  : 2445.430
cache size : 512 KB
physical id : 0
---
fpu  : yes
fpu_exception : yes
cpuid level : 13
wp  : yes
flags  : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext invpcid_single vmmcall fsgsbase bmi1 avx2 smep bmi2 invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid
bugs  : sysret_ss_attrs null_seg spectre_v1 spectre_v2 spec_store_bypass
bogomips : 4890.86
TLB size : 2560 4K pages
cache_alignment : 64
address sizes : 48 bits physical, 48 bits virtual
power management:


processor : 4
vendor_id : AuthenticAMD
cpu family : 25
model  : 1
model name : AMD EPYC 7763 64-Core Processor
microcode : 0xffffffff
cpu MHz  : 2445.430
cache size : 512 KB
physical id : 0
---
fpu  : yes
fpu_exception : yes
cpuid level : 13
wp  : yes
flags  : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext invpcid_single vmmcall fsgsbase bmi1 avx2 smep bmi2 invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid
bugs  : sysret_ss_attrs null_seg spectre_v1 spectre_v2 spec_store_bypass
bogomips : 4890.86
TLB size : 2560 4K pages
cache_alignment : 64
address sizes : 48 bits physical, 48 bits virtual
power management:


processor : 5
vendor_id : AuthenticAMD
cpu family : 25
model  : 1
model name : AMD EPYC 7763 64-Core Processor
microcode : 0xffffffff
cpu MHz  : 2445.430
cache size : 512 KB
physical id : 0
---
fpu  : yes
fpu_exception : yes
cpuid level : 13
wp  : yes
flags  : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext invpcid_single vmmcall fsgsbase bmi1 avx2 smep bmi2 invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid
bugs  : sysret_ss_attrs null_seg spectre_v1 spectre_v2 spec_store_bypass
bogomips : 4890.86
TLB size : 2560 4K pages
cache_alignment : 64
address sizes : 48 bits physical, 48 bits virtual
power management:


processor : 6
vendor_id : AuthenticAMD
cpu family : 25
model  : 1
model name : AMD EPYC 7763 64-Core Processor
microcode : 0xffffffff
cpu MHz  : 2445.430
cache size : 512 KB
physical id : 0
---
fpu  : yes
fpu_exception : yes
cpuid level : 13
wp  : yes
flags  : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext invpcid_single vmmcall fsgsbase bmi1 avx2 smep bmi2 invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid
bugs  : sysret_ss_attrs null_seg spectre_v1 spectre_v2 spec_store_bypass
bogomips : 4890.86
TLB size : 2560 4K pages
cache_alignment : 64
address sizes : 48 bits physical, 48 bits virtual
power management:


processor : 7
vendor_id : AuthenticAMD
cpu family : 25
model  : 1
model name : AMD EPYC 7763 64-Core Processor
microcode : 0xffffffff
cpu MHz  : 2445.430
cache size : 512 KB
physical id : 0
---
fpu  : yes
fpu_exception : yes
cpuid level : 13
wp  : yes
flags  : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext invpcid_single vmmcall fsgsbase bmi1 avx2 smep bmi2 invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid
bugs  : sysret_ss_attrs null_seg spectre_v1 spectre_v2 spec_store_bypass
bogomips : 4890.86
TLB size : 2560 4K pages
cache_alignment : 64
address sizes : 48 bits physical, 48 bits virtual
power management:


processor : 8
vendor_id : AuthenticAMD
cpu family : 25
model  : 1
model name : AMD EPYC 7763 64-Core Processor
microcode : 0xffffffff
cpu MHz  : 2431.302
cache size : 512 KB
physical id : 0
---
fpu  : yes
fpu_exception : yes
cpuid level : 13
wp  : yes
flags  : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext invpcid_single vmmcall fsgsbase bmi1 avx2 smep bmi2 invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid
bugs  : sysret_ss_attrs null_seg spectre_v1 spectre_v2 spec_store_bypass
bogomips : 4890.86
TLB size : 2560 4K pages
cache_alignment : 64
address sizes : 48 bits physical, 48 bits virtual
power management:


processor : 9
vendor_id : AuthenticAMD
cpu family : 25
model  : 1
model name : AMD EPYC 7763 64-Core Processor
microcode : 0xffffffff
cpu MHz  : 2445.430
cache size : 512 KB
physical id : 0
---
fpu  : yes
fpu_exception : yes
cpuid level : 13
wp  : yes
flags  : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext invpcid_single vmmcall fsgsbase bmi1 avx2 smep bmi2 invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid
bugs  : sysret_ss_attrs null_seg spectre_v1 spectre_v2 spec_store_bypass
bogomips : 4890.86
TLB size : 2560 4K pages
cache_alignment : 64
address sizes : 48 bits physical, 48 bits virtual
power management:


processor : 10
vendor_id : AuthenticAMD
cpu family : 25
model  : 1
model name : AMD EPYC 7763 64-Core Processor
microcode : 0xffffffff
cpu MHz  : 2445.430
cache size : 512 KB
physical id : 0
---
fpu  : yes
fpu_exception : yes
cpuid level : 13
wp  : yes
flags  : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext invpcid_single vmmcall fsgsbase bmi1 avx2 smep bmi2 invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid
bugs  : sysret_ss_attrs null_seg spectre_v1 spectre_v2 spec_store_bypass
bogomips : 4890.86
TLB size : 2560 4K pages
cache_alignment : 64
address sizes : 48 bits physical, 48 bits virtual
power management:


processor : 11
vendor_id : AuthenticAMD
cpu family : 25
model  : 1
model name : AMD EPYC 7763 64-Core Processor
microcode : 0xffffffff
cpu MHz  : 2445.430
cache size : 512 KB
physical id : 0
---
fpu  : yes
fpu_exception : yes
cpuid level : 13
wp  : yes
flags  : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext invpcid_single vmmcall fsgsbase bmi1 avx2 smep bmi2 invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid
bugs  : sysret_ss_attrs null_seg spectre_v1 spectre_v2 spec_store_bypass
bogomips : 4890.86
TLB size : 2560 4K pages
cache_alignment : 64
address sizes : 48 bits physical, 48 bits virtual
power management:


processor : 12
vendor_id : AuthenticAMD
cpu family : 25
model  : 1
model name : AMD EPYC 7763 64-Core Processor
microcode : 0xffffffff
cpu MHz  : 2445.430
cache size : 512 KB
physical id : 0
---
fpu  : yes
fpu_exception : yes
cpuid level : 13
wp  : yes
flags  : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext invpcid_single vmmcall fsgsbase bmi1 avx2 smep bmi2 invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid
bugs  : sysret_ss_attrs null_seg spectre_v1 spectre_v2 spec_store_bypass
bogomips : 4890.86
TLB size : 2560 4K pages
cache_alignment : 64
address sizes : 48 bits physical, 48 bits virtual
power management:


processor : 13
vendor_id : AuthenticAMD
cpu family : 25
model  : 1
model name : AMD EPYC 7763 64-Core Processor
microcode : 0xffffffff
cpu MHz  : 2445.430
cache size : 512 KB
physical id : 0
---
fpu  : yes
fpu_exception : yes
cpuid level : 13
wp  : yes
flags  : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext invpcid_single vmmcall fsgsbase bmi1 avx2 smep bmi2 invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid
bugs  : sysret_ss_attrs null_seg spectre_v1 spectre_v2 spec_store_bypass
bogomips : 4890.86
TLB size : 2560 4K pages
cache_alignment : 64
address sizes : 48 bits physical, 48 bits virtual
power management:


processor : 14
vendor_id : AuthenticAMD
cpu family : 25
model  : 1
model name : AMD EPYC 7763 64-Core Processor
microcode : 0xffffffff
cpu MHz  : 3116.667
cache size : 512 KB
physical id : 0
---
fpu  : yes
fpu_exception : yes
cpuid level : 13
wp  : yes
flags  : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext invpcid_single vmmcall fsgsbase bmi1 avx2 smep bmi2 invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid
bugs  : sysret_ss_attrs null_seg spectre_v1 spectre_v2 spec_store_bypass
bogomips : 4890.86
TLB size : 2560 4K pages
cache_alignment : 64
address sizes : 48 bits physical, 48 bits virtual
power management:


processor : 15
vendor_id : AuthenticAMD
cpu family : 25
model  : 1
model name : AMD EPYC 7763 64-Core Processor
microcode : 0xffffffff
cpu MHz  : 2445.430
cache size : 512 KB
physical id : 0
---
fpu  : yes
fpu_exception : yes
cpuid level : 13
wp  : yes
flags  : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext invpcid_single vmmcall fsgsbase bmi1 avx2 smep bmi2 invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid
bugs  : sysret_ss_attrs null_seg spectre_v1 spectre_v2 spec_store_bypass
bogomips : 4890.86
TLB size : 2560 4K pages
cache_alignment : 64
address sizes : 48 bits physical, 48 bits virtual
power management:

---
Building stage1 library artifacts (x86_64-unknown-linux-gnu) 
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","xtensa")' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Csplit-debuginfo=off -Cprefer-dynamic -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' --target x86_64-unknown-linux-gnu --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=split-debuginfo --print=crate-name --print=cfg` (exit status: 254)
  --- stderr
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0x97cff3)[0x7f5464443ff3]
  /lib/x86_64-linux-gnu/libc.so.6(+0x42520)[0x7f546371b520]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-16-rust-1.70.0-nightly.so(+0x7262e06)[0x7f5461e62e06]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0xd10d87)[0x7f54647d7d87]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0xbc960c)[0x7f546469060c]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0xbc8744)[0x7f546468f744]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(_RNvXs5_Cs7DTEPoQIU0M_18rustc_codegen_llvmNtB5_18LlvmCodegenBackendNtNtNtCsiydnvZ3aRh1_17rustc_codegen_ssa6traits7backend14CodegenBackend15target_features+0x1b)[0x7f54646ef77b]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(_RNvNtCsbMR7lNZiyEF_15rustc_interface4util17add_configuration+0x37)[0x7f5464531837]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(_RNvNtCsbMR7lNZiyEF_15rustc_interface4util14create_session+0x89d)[0x7f54645322dd]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0x987540)[0x7f546444e540]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0x9822c0)[0x7f54644492c0]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0x983e03)[0x7f546444ae03]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-2268eb07c4da6a07.so(rust_metadata_std_e292dd3bffb96032+0xca46e)[0x7f54639cf46e]
  /lib/x86_64-linux-gnu/libc.so.6(+0x94b43)[0x7f546376db43]
  /lib/x86_64-linux-gnu/libc.so.6(+0x126a00)[0x7f54637ffa00]
  rustc exited with signal: 11 (SIGSEGV) (core dumped)
cat: /tmp/toolstate/toolstates.json: No such file or directory
