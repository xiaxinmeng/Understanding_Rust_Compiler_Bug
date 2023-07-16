
The CLOCK_MONOTONIC clock is not affected by discontinuous
jumps in the system time (e.g., if the system
administrator manually changes the clock), but is affected
by the incremental adjustments performed by adjtime(3) and
NTP.  This clock does not count time that the system is
suspended.  All CLOCK_MONOTONIC variants guarantee that
the time returned by consecutive calls will not go
backwards, but successive calls may—depending on the
architecture—return identical (not-increased) time values.
