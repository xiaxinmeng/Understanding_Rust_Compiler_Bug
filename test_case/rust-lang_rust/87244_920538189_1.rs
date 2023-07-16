bash
cg_diff results/cgout-{before,after}-stm32f4-Check-Full | \
sed 's@da7d405357600a76f2b93b8aa41fe5ee5da7885d@@' | \
        sed 's@718d53b0cb7dde93499cb92950d60b412f5a3d05@@' | \
        rustfilt | \
        cg_annotate /dev/stdin | less
