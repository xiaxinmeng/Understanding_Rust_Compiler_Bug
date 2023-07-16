 make
BASE_PANDOC_OPTS := -a -b -c
TEX_OPTS = $(BASE_PANDOC_OPTS) -d -e -f
HTML_OPTS = $(BASE_PANDOC_OPTS) -h -i -j
