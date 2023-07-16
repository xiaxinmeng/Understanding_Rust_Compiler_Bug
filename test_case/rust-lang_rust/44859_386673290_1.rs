
Module                  Size  Used by
vboxsf                 45056  1
binfmt_misc            20480  1
intel_powerclamp       16384  0
crct10dif_pclmul       16384  0
crc32_pclmul           16384  0
ghash_clmulni_intel    16384  0
vboxvideo              36864  1
ttm                   106496  1 vboxvideo
intel_rapl_perf        16384  0
drm_kms_helper        167936  1 vboxvideo
input_leds             16384  0
serio_raw              16384  0
drm                   401408  4 vboxvideo,ttm,drm_kms_helper
fb_sys_fops            16384  1 drm_kms_helper
syscopyarea            16384  2 vboxvideo,drm_kms_helper
sysfillrect            16384  2 vboxvideo,drm_kms_helper
sysimgblt              16384  2 vboxvideo,drm_kms_helper
vboxguest             303104  2 vboxsf
snd_intel8x0           40960  0
snd_ac97_codec        131072  1 snd_intel8x0
ac97_bus               16384  1 snd_ac97_codec
snd_pcm                98304  2 snd_ac97_codec,snd_intel8x0
snd_timer              32768  1 snd_pcm
snd                    81920  4 snd_ac97_codec,snd_timer,snd_intel8x0,snd_pcm
soundcore              16384  1 snd
mac_hid                16384  0
sch_fq_codel           20480  2
ib_iser                49152  0
rdma_cm                61440  1 ib_iser
iw_cm                  45056  1 rdma_cm
ib_cm                  53248  1 rdma_cm
ib_core               225280  4 ib_iser,ib_cm,rdma_cm,iw_cm
iscsi_tcp              20480  0
libiscsi_tcp           20480  1 iscsi_tcp
libiscsi               53248  3 ib_iser,libiscsi_tcp,iscsi_tcp
scsi_transport_iscsi    98304  4 ib_iser,libiscsi,iscsi_tcp
ip_tables              28672  0
x_tables               40960  1 ip_tables
autofs4                40960  2
btrfs                1122304  0
zstd_compress         163840  1 btrfs
raid10                 53248  0
raid456               143360  0
async_raid6_recov      20480  1 raid456
async_memcpy           16384  2 raid456,async_raid6_recov
async_pq               16384  2 raid456,async_raid6_recov
async_xor              16384  3 async_pq,raid456,async_raid6_recov
async_tx               16384  5 async_xor,async_pq,raid456,async_memcpy,async_raid6_recov
xor                    24576  2 async_xor,btrfs
raid6_pq              114688  4 async_pq,btrfs,raid456,async_raid6_recov
libcrc32c              16384  1 raid456
raid1                  40960  0
raid0                  20480  0
multipath              16384  0
linear                 16384  0
aesni_intel           188416  0
aes_x86_64             20480  1 aesni_intel
crypto_simd            16384  1 aesni_intel
cryptd                 24576  3 crypto_simd,ghash_clmulni_intel,aesni_intel
glue_helper            16384  1 aesni_intel
psmouse               147456  0
ahci                   36864  1
libahci                32768  1 ahci
i2c_piix4              24576  0
e1000                 143360  0
video                  40960  0
pata_acpi              16384  0
