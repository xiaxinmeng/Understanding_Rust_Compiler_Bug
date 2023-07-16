
$ for f in $(cat y); do curl -sf http://$f/centos/5/os/x86_64/repodata/ >/dev/null || echo bad $f; done
bad 108.61.16.227
bad 160.10.26.25
