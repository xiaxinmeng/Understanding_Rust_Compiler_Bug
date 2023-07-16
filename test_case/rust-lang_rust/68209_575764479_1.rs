dmesg
[2254946.092252] oom-kill:constraint=CONSTRAINT_NONE,nodemask=(null),cpuset=/,mems_allowed=0,global_oom,task_memcg=/user.slice/user-1000.slice/session-93.scope,task=rustc,pid=136409,uid=1000
[2254946.092262] Out of memory: Killed process 136409 (rustc) total-vm:134493012kB, anon-rss:31864240kB, file-rss:0kB, shmem-rss:0kB
[2254949.915967] oom_reaper: reaped process 136409 (rustc), now anon-rss:0kB, file-rss:0kB, shmem-rss:0kB
