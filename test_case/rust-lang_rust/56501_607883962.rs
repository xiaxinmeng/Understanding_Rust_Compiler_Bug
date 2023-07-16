
$ dmesg -T | grep -e oom -e "Out of memory" 
[Thu Apr  2 15:52:54 2020] kthreadd invoked oom-killer: gfp_mask=0x2dc2(GFP_KERNEL|__GFP_HIGHMEM|__GFP_NOWARN|__GFP_ZERO), order=0, oom_score_adj=0
[Thu Apr  2 15:52:54 2020]  oom_kill_process+0x188/0x190
[Thu Apr  2 15:52:54 2020] [  pid  ]   uid  tgid total_vm      rss pgtables_bytes swapents oom_score_adj name
[Thu Apr  2 15:52:54 2020] oom-kill:constraint=CONSTRAINT_NONE,nodemask=(null),cpuset=/,mems_allowed=0,global_oom,task_memcg=/user.slice,task=rustc,pid=2829,uid=1000
[Thu Apr  2 15:52:54 2020] Out of memory: Killed process 2829 (rustc) total-vm:365980kB, anon-rss:101964kB, file-rss:0kB, shmem-rss:0kB, UID:1000 pgtables:576kB oom_score_adj:0
[Thu Apr  2 15:52:54 2020] oom_reaper: reaped process 2829 (rustc), now anon-rss:0kB, file-rss:0kB, shmem-rss:0kB

