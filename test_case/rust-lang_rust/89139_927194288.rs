bash
./target/release/collector diff_local self-profile +30278d3cf92b581550933546370443a5d5700002 +67365d64bcdfeae1334bf2ff49587c27d1c973f0 --include ctfe-stress-4 --builds Check --runs Full
mmview ./results/Zsp-30278d3cf92b581550933546370443a5d5700002-ctfe-stress-4-Check-Full/Zsp.mm_profdata | rg 'label: specializes' -A1 | rg additional_data > before
mmview ./results/Zsp-67365d64bcdfeae1334bf2ff49587c27d1c973f0-ctfe-stress-4-Check-Full/Zsp.mm_profdata | rg 'label: specializes' -A1 | rg additional_data > after
diff -u <(sort ./base) <(sort ./after) > query.diff
