
9dc396747b31a7c3918d44bd30be115360712f9f is the first bad commit
commit 9dc396747b31a7c3918d44bd30be115360712f9f
Author: Ariel Ben-Yehuda <ariel.byd@gmail.com>
Date:   Tue Nov 28 01:45:16 2017 +0200

    funnel all unwind paths through a single Resume block

    This simplifies analysis and borrow-checking because liveness at the
    resume point can always be simply propagated.

    Later on, the "dead" Resumes are removed.

:040000 040000 6d09d3a9b31b20377395eaee88ec98fb0c55b604 b2a7650815c8a9b4dc844778da015e3f71a059a0 M	src
