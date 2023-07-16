
for l in `rg CFG_ config.mk | awk '{ print $1 }'`; do rg -q $l src/bootstrap/ || echo "$l" ; done
