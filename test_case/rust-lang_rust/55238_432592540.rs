sh
% cd jemallocator/jemalloc-sys
% paste <(git log -p jemalloc|grep Date|grep -v 'Fri Apr 27') <(for c in $(git log -p jemalloc|grep '^+S'|cut -d' ' -f3); (cd jemalloc; echo -n "$c "; git show $c:ChangeLog|head|grep \*))
Date:   Wed May 16 19:16:50 2018 +0200	61efbda7098de6fe64c362d309824864308c36d4 * 5.1.0 (May 4th, 2018)
Date:   Thu May 3 16:37:24 2018 +0200	b8f4c730eff28edee4b583ff5b6ee1fac0f26c27 * 5.0.1 (July 1, 2017)
Date:   Thu May 3 00:23:40 2018 +0200	b8f4c730eff28edee4b583ff5b6ee1fac0f26c27 * 5.0.1 (July 1, 2017)
Date:   Thu Nov 23 19:08:53 2017 +0100	3c8e8db1b57e8101c33b2738213aad07cecbb3d3 * 5.0.1 (July 1, 2017)
Date:   Thu Jul 27 11:07:15 2017 -0300	b6bdb328cf73b67129f7e0a92a513d158763aadb * 4.5.0 (February 28, 2017)
Date:   Mon May 29 21:03:47 2017 +0800	3288e0659c08fb5006f6d6dd4b5675ed0c2c432a * 4.5.0 (February 28, 2017)
Date:   Thu Mar 30 22:15:07 2017 +0800	11bfb0dcf85f7aa92abd30524bb1e42e18d108c6 * 4.1.0 (February 28, 2016)
Date:   Mon Mar 7 13:15:29 2016 -0800	aab1c0a0e0b39825b16673128729ef46310a5da8 * 4.1.0 (February 28, 2016)
Date:   Wed Jan 20 17:03:24 2016 -0800	f84e30927284b0c500ed3eaf09e8e159da20ddaf * 4.0.4 (October 24, 2015)