
$ c=83ffe9cde7fe0b4deb0d1b54175fd9b19c38179c && git log -1 "$c" && git show --raw "$c" | wc -l
commit 83ffe9cde7fe0b4deb0d1b54175fd9b19c38179c
Author: Jakub Jelinek <jakub@redhat.com>
Date:   Mon Jan 16 11:50:43 2023 +0100

    Update copyright years.
16071
$ contrib/gcc-changelog/git_check_commit.py "$c"
Checking 83ffe9cde7fe0b4deb0d1b54175fd9b19c38179c: OK
