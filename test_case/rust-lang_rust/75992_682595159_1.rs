bash
for n in $(seq 15); do sed "s/REPLACE_ME/$n/g" template.rs > main.rs && echo && echo && echo && echo "== Level $n @ 1.45.0" && time rustc +1.45.0 --edition 2018 main.rs && echo "== Level $n @ 1.46.0" && time rustc --edition 2018 main.rs; done
