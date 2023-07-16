
statx(AT_FDCWD, "/sys/class/power_supply/CROS_USBPD_CHARGER0/status", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_BASIC_STATS|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0444, stx_size=4096, ...}) = 0
openat(AT_FDCWD, "/sys/class/power_supply/CROS_USBPD_CHARGER0/status", O_RDONLY|O_CLOEXEC) = 10
statx(10, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_BASIC_STATS|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0444, stx_size=4096, ...}) = 0
lseek(10, 0, SEEK_CUR)                  = 0
read(10, "Not charging\n", 4096)        = 13
read(10, "", 4083)                      = 0
close(10)                               = 0
statx(AT_FDCWD, "/sys/class/power_supply/CROS_USBPD_CHARGER1/online", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_BASIC_STATS|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0444, stx_size=4096, ...}) = 0
