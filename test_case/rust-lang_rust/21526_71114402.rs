
==26307== Invalid read of size 8
==26307==    at 0x4E4B4D5: collections..vec..Vec$LT$syntax..ptr..P$LT$syntax..codemap..Spanned$LT$syntax..ast..MetaItem_$GT$$GT$$GT$::glue_drop.12051::h50b8b9a8c4be2bb3 (in /home/huon/rust3/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-4e7c5e5c.so)
==26307==    by 0xF90490F: ???
==26307==    by 0x2F: ???
==26307==    by 0xD461B9F: ???
==26307==    by 0xD52354F: ???
==26307==    by 0xF5F963F: ???
==26307==    by 0x4E4E8FF: syntax..codemap..Spanned$LT$syntax..ast..Attribute_$GT$::glue_drop.12756::hb78b33cc126cc208 (in /home/huon/rust3/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-4e7c5e5c.so)
==26307==    by 0x8097101EF: ???
==26307==    by 0xC9B2F8F: ???
==26307==    by 0xD461B6F: ???
==26307==    by 0x4EA9E2E: iter::FlatMap$LT$A$C$$u{20}B$C$$u{20}I$C$$u{20}U$C$$u{20}F$GT$.Iterator::next::h11721808144370039366 (in /home/huon/rust3/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-4e7c5e5c.so)
==26307==    by 0x4E9746C: fold::noop_fold_item::h5513282176331594077 (in /home/huon/rust3/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-4e7c5e5c.so)
==26307==  Address 0xe5f3240 is 0 bytes inside a block of size 80 free'd
==26307==    at 0x4C29E90: free (vg_replace_malloc.c:473)
==26307==    by 0x4E4B597: collections..vec..Vec$LT$syntax..ptr..P$LT$syntax..codemap..Spanned$LT$syntax..ast..MetaItem_$GT$$GT$$GT$::glue_drop.12051::h50b8b9a8c4be2bb3 (in /home/huon/rust3/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-4e7c5e5c.so)
==26307==    by 0xF90490F: ???
==26307==    by 0xD46183F: ???
==26307==    by 0xF5F963F: ???
==26307==    by 0xD52354F: ???
==26307==    by 0xC9B2EFF: ???
==26307==    by 0x4E4E8FF: syntax..codemap..Spanned$LT$syntax..ast..Attribute_$GT$::glue_drop.12756::hb78b33cc126cc208 (in /home/huon/rust3/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-4e7c5e5c.so)
==26307==    by 0x4F: ???
==26307==    by 0xC9B2F8F: ???
==26307==    by 0xD4618AF: ???
==26307==    by 0x4EAADA8: fold::noop_fold_impl_item::h7121156065768389895 (in /home/huon/rust3/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-4e7c5e5c.so)
==26307== 
==26307== Invalid read of size 8
==26307==    at 0x4E4B55F: collections..vec..Vec$LT$syntax..ptr..P$LT$syntax..codemap..Spanned$LT$syntax..ast..MetaItem_$GT$$GT$$GT$::glue_drop.12051::h50b8b9a8c4be2bb3 (in /home/huon/rust3/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-4e7c5e5c.so)
==26307==    by 0xF90490F: ???
==26307==    by 0x2F: ???
==26307==    by 0xD461B9F: ???
==26307==    by 0xD52354F: ???
==26307==    by 0xF5F963F: ???
==26307==    by 0x4E4E8FF: syntax..codemap..Spanned$LT$syntax..ast..Attribute_$GT$::glue_drop.12756::hb78b33cc126cc208 (in /home/huon/rust3/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-4e7c5e5c.so)
==26307==    by 0x8097101EF: ???
==26307==    by 0xC9B2F8F: ???
==26307==    by 0xD461B6F: ???
==26307==    by 0x4EA9E2E: iter::FlatMap$LT$A$C$$u{20}B$C$$u{20}I$C$$u{20}U$C$$u{20}F$GT$.Iterator::next::h11721808144370039366 (in /home/huon/rust3/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-4e7c5e5c.so)
==26307==    by 0x4E9746C: fold::noop_fold_item::h5513282176331594077 (in /home/huon/rust3/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-4e7c5e5c.so)
==26307==  Address 0xe5f3248 is 8 bytes inside a block of size 80 free'd
==26307==    at 0x4C29E90: free (vg_replace_malloc.c:473)
==26307==    by 0x4E4B597: collections..vec..Vec$LT$syntax..ptr..P$LT$syntax..codemap..Spanned$LT$syntax..ast..MetaItem_$GT$$GT$$GT$::glue_drop.12051::h50b8b9a8c4be2bb3 (in /home/huon/rust3/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-4e7c5e5c.so)
==26307==    by 0xF90490F: ???
==26307==    by 0xD46183F: ???
==26307==    by 0xF5F963F: ???
==26307==    by 0xD52354F: ???
==26307==    by 0xC9B2EFF: ???
==26307==    by 0x4E4E8FF: syntax..codemap..Spanned$LT$syntax..ast..Attribute_$GT$::glue_drop.12756::hb78b33cc126cc208 (in /home/huon/rust3/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-4e7c5e5c.so)
==26307==    by 0x4F: ???
==26307==    by 0xC9B2F8F: ???
==26307==    by 0xD4618AF: ???
==26307==    by 0x4EAADA8: fold::noop_fold_impl_item::h7121156065768389895 (in /home/huon/rust3/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-4e7c5e5c.so)
