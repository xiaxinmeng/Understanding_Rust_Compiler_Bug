bash
for n in $(seq 29); do sed "s/REPLACE_ME/$n/g" template.rs > main.rs && echo "== Level $n" && time rustc --edition 2018 main.rs; done
