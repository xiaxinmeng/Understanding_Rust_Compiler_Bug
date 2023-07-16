
`is_loopback` doesn't detect IPv4 loopback addresses represented as IPv4-mapped IPv6 addresses.

To detect those, use `to_canonical().is_loopback()`.
