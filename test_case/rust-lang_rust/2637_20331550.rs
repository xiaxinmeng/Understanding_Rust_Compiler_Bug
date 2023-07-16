
strptime("1999-05-31T13:20:00-0500", "%FT%T%z")
-> Ok({tm_sec: 0, tm_min: 20, tm_hour: 13, tm_mday: 31, tm_mon: 4, tm_year: 99, tm_wday: 0, tm_yday: 0, tm_isdst: 0, tm_gmtoff: 0, tm_zone: ~"", tm_nsec: 0})

strptime("1999-05-31T13:20:00-0400", "%FT%T%z")
-> Ok({tm_sec: 0, tm_min: 20, tm_hour: 13, tm_mday: 31, tm_mon: 4, tm_year: 99, tm_wday: 0, tm_yday: 0, tm_isdst: 0, tm_gmtoff: 0, tm_zone: ~"", tm_nsec: 0})
