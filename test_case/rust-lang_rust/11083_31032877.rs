
(gdb) bt
#0  rust_begin_unwind (token=839147) at /home/jdm/sdb/rust/src/rt/rust_cxx_glue.cpp:29
#1  0x00007ffff786e228 in rt::task::Unwinder::begin_unwind::h1b02db942e8abc23cf4e006d791088b60ed985b5f15ea5d664ff356de5d7199efeaI::v0.9.pre () from /usr/local/bin/../lib/libstd-04ff901e-0.9-pre.so
#2  0x00007ffff686cada in rt::task::begin_unwind::h9ef1f86da8c723b2f711f15c54e1d13c1248b66ff5fc6505fce81450b7c573b7aM::v0.9.pre () from /usr/local/bin/../lib/libsyntax-2e4c0458-0.9-pre.so
#3  0x00007ffff687a90d in diagnostic::span_handler$CodemapT::span_fatal::h45b761fbcef600876cf6331eff99bffed9ca5c091e344090c9f94785e2c1129drcaO::v0.9.pre ()
   from /usr/local/bin/../lib/libsyntax-2e4c0458-0.9-pre.so
#4  0x00007ffff687bb82 in diagnostic::span_handler$CodemapT::span_bug::hee6f584f50040e1ca7f4c4aaf6935d96a62eaab0ba67b9148676cd0df440c77arca9::v0.9.pre () from /usr/local/bin/../lib/libsyntax-2e4c0458-0.9-pre.so
#5  0x00007ffff441b086 in driver::session::Session_::span_bug::h2f69a2f9bbcd90667801663bc3645a067356a2f04ba81f139bc7a72c1d82aaf4usa3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#6  0x00007ffff457a4a9 in middle::trans::debuginfo::set_members_of_composite_type::hce31eaf2a96f7981b504889be9da0dda65c8b9d71b8de123f62e292521067da6aS::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#7  0x00007ffff4579610 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#8  0x00007ffff45743ce in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#9  0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#10 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#11 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#12 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#13 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#14 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#15 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#16 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#17 0x00007ffff457454c in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#18 0x00007ffff45806d5 in middle::trans::debuginfo::subroutine_type_metadata::ha5a1676df1bf03124dab60afe77c8b2175f365a1a2283e1983706fe1480ede42aB::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#19 0x00007ffff45745f8 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#20 0x00007ffff457c4c5 in middle::trans::debuginfo::MemberDescriptionFactory$EnumVariantMemberDescriptionFactory::create_member_descriptions::h2e78253ab000ae1701de1bfbbaffa0843f2ff76e9c1bfd68026762b7579d0b73H7aa::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#21 0x00007ffff457ae66 in middle::trans::debuginfo::MemberDescriptionFactory$GeneralMemberDescriptionFactory::create_member_descriptions::h91c371565c4be6372073d67a738909bfd1e67a95a797ecd3b33fb85a876ed718s3av::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#22 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#23 0x00007ffff45743ce in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#24 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#25 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#26 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#27 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#28 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#29 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#30 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#31 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#32 0x00007ffff457454c in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#33 0x00007ffff45806d5 in middle::trans::debuginfo::subroutine_type_metadata::ha5a1676df1bf03124dab60afe77c8b2175f365a1a2283e1983706fe1480ede42aB::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#34 0x00007ffff45745f8 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#35 0x00007ffff457c4c5 in middle::trans::debuginfo::MemberDescriptionFactory$EnumVariantMemberDescriptionFactory::create_member_descriptions::h2e78253ab000ae1701de1bfbbaffa0843f2ff76e9c1bfd68026762b7579d0b73H7aa::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#36 0x00007ffff457ae66 in middle::trans::debuginfo::MemberDescriptionFactory$GeneralMemberDescriptionFactory::create_member_descriptions::h91c371565c4be6372073d67a738909bfd1e67a95a797ecd3b33fb85a876ed718s3av::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#37 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
---Type <return> to continue, or q <return> to quit---
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#38 0x00007ffff45743ce in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#39 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#40 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#41 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#42 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#43 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#44 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#45 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#46 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#47 0x00007ffff457454c in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#48 0x00007ffff45806d5 in middle::trans::debuginfo::subroutine_type_metadata::ha5a1676df1bf03124dab60afe77c8b2175f365a1a2283e1983706fe1480ede42aB::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#49 0x00007ffff45745f8 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#50 0x00007ffff457c4c5 in middle::trans::debuginfo::MemberDescriptionFactory$EnumVariantMemberDescriptionFactory::create_member_descriptions::h2e78253ab000ae1701de1bfbbaffa0843f2ff76e9c1bfd68026762b7579d0b73H7aa::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#51 0x00007ffff457ae66 in middle::trans::debuginfo::MemberDescriptionFactory$GeneralMemberDescriptionFactory::create_member_descriptions::h91c371565c4be6372073d67a738909bfd1e67a95a797ecd3b33fb85a876ed718s3av::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#52 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#53 0x00007ffff45743ce in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#54 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#55 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#56 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#57 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#58 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#59 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#60 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#61 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#62 0x00007ffff457454c in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#63 0x00007ffff45806d5 in middle::trans::debuginfo::subroutine_type_metadata::ha5a1676df1bf03124dab60afe77c8b2175f365a1a2283e1983706fe1480ede42aB::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#64 0x00007ffff45745f8 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#65 0x00007ffff457c4c5 in middle::trans::debuginfo::MemberDescriptionFactory$EnumVariantMemberDescriptionFactory::create_member_descriptions::h2e78253ab000ae1701de1bfbbaffa0843f2ff76e9c1bfd68026762b7579d0b73H7aa::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#66 0x00007ffff457ae66 in middle::trans::debuginfo::MemberDescriptionFactory$GeneralMemberDescriptionFactory::create_member_descriptions::h91c371565c4be6372073d67a738909bfd1e67a95a797ecd3b33fb85a876ed718s3av::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#67 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#68 0x00007ffff45743ce in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#69 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#70 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#71 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#72 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#73 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#74 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.---Type <return> to continue, or q <return> to quit---
9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#75 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#76 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#77 0x00007ffff457454c in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#78 0x00007ffff45806d5 in middle::trans::debuginfo::subroutine_type_metadata::ha5a1676df1bf03124dab60afe77c8b2175f365a1a2283e1983706fe1480ede42aB::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#79 0x00007ffff45745f8 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#80 0x00007ffff457c4c5 in middle::trans::debuginfo::MemberDescriptionFactory$EnumVariantMemberDescriptionFactory::create_member_descriptions::h2e78253ab000ae1701de1bfbbaffa0843f2ff76e9c1bfd68026762b7579d0b73H7aa::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#81 0x00007ffff457ae66 in middle::trans::debuginfo::MemberDescriptionFactory$GeneralMemberDescriptionFactory::create_member_descriptions::h91c371565c4be6372073d67a738909bfd1e67a95a797ecd3b33fb85a876ed718s3av::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#82 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#83 0x00007ffff45743ce in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#84 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#85 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#86 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#87 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#88 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#89 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#90 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#91 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#92 0x00007ffff457454c in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#93 0x00007ffff45806d5 in middle::trans::debuginfo::subroutine_type_metadata::ha5a1676df1bf03124dab60afe77c8b2175f365a1a2283e1983706fe1480ede42aB::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#94 0x00007ffff45745f8 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#95 0x00007ffff457c4c5 in middle::trans::debuginfo::MemberDescriptionFactory$EnumVariantMemberDescriptionFactory::create_member_descriptions::h2e78253ab000ae1701de1bfbbaffa0843f2ff76e9c1bfd68026762b7579d0b73H7aa::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#96 0x00007ffff457ae66 in middle::trans::debuginfo::MemberDescriptionFactory$GeneralMemberDescriptionFactory::create_member_descriptions::h91c371565c4be6372073d67a738909bfd1e67a95a797ecd3b33fb85a876ed718s3av::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#97 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#98 0x00007ffff45743ce in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#99 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#100 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#101 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#102 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#103 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#104 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#105 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#106 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#107 0x00007ffff457454c in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#108 0x00007ffff45806d5 in middle::trans::debuginfo::subroutine_type_metadata::ha5a1676df1bf03124dab60afe77c8b2175f365a1a2283e1983706fe1480ede42aB::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#109 0x00007ffff45745f8 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#110 0x00007ffff457c4c5 in middle::trans::debuginfo::MemberDescriptionFactory$EnumVariantMemberDescriptionFactory::create_member_descriptions::h2e78253ab000ae1701de1bfbbaffa0843f2ff76e9c1bfd68026762b7579d0b73H7aa::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
---Type <return> to continue, or q <return> to quit---
#111 0x00007ffff457ae66 in middle::trans::debuginfo::MemberDescriptionFactory$GeneralMemberDescriptionFactory::create_member_descriptions::h91c371565c4be6372073d67a738909bfd1e67a95a797ecd3b33fb85a876ed718s3av::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#112 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#113 0x00007ffff45743ce in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#114 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#115 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#116 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#117 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#118 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#119 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#120 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#121 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#122 0x00007ffff457454c in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#123 0x00007ffff45806d5 in middle::trans::debuginfo::subroutine_type_metadata::ha5a1676df1bf03124dab60afe77c8b2175f365a1a2283e1983706fe1480ede42aB::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#124 0x00007ffff45745f8 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#125 0x00007ffff457c4c5 in middle::trans::debuginfo::MemberDescriptionFactory$EnumVariantMemberDescriptionFactory::create_member_descriptions::h2e78253ab000ae1701de1bfbbaffa0843f2ff76e9c1bfd68026762b7579d0b73H7aa::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#126 0x00007ffff457ae66 in middle::trans::debuginfo::MemberDescriptionFactory$GeneralMemberDescriptionFactory::create_member_descriptions::h91c371565c4be6372073d67a738909bfd1e67a95a797ecd3b33fb85a876ed718s3av::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#127 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#128 0x00007ffff45743ce in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#129 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#130 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#131 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#132 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#133 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#134 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#135 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#136 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#137 0x00007ffff457454c in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#138 0x00007ffff45806d5 in middle::trans::debuginfo::subroutine_type_metadata::ha5a1676df1bf03124dab60afe77c8b2175f365a1a2283e1983706fe1480ede42aB::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#139 0x00007ffff45745f8 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#140 0x00007ffff457c4c5 in middle::trans::debuginfo::MemberDescriptionFactory$EnumVariantMemberDescriptionFactory::create_member_descriptions::h2e78253ab000ae1701de1bfbbaffa0843f2ff76e9c1bfd68026762b7579d0b73H7aa::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#141 0x00007ffff457ae66 in middle::trans::debuginfo::MemberDescriptionFactory$GeneralMemberDescriptionFactory::create_member_descriptions::h91c371565c4be6372073d67a738909bfd1e67a95a797ecd3b33fb85a876ed718s3av::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#142 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#143 0x00007ffff45743ce in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#144 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#145 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#146 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
---Type <return> to continue, or q <return> to quit---
#147 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#148 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#149 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#150 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#151 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#152 0x00007ffff457454c in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#153 0x00007ffff45806d5 in middle::trans::debuginfo::subroutine_type_metadata::ha5a1676df1bf03124dab60afe77c8b2175f365a1a2283e1983706fe1480ede42aB::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#154 0x00007ffff45745f8 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#155 0x00007ffff457c4c5 in middle::trans::debuginfo::MemberDescriptionFactory$EnumVariantMemberDescriptionFactory::create_member_descriptions::h2e78253ab000ae1701de1bfbbaffa0843f2ff76e9c1bfd68026762b7579d0b73H7aa::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#156 0x00007ffff457ae66 in middle::trans::debuginfo::MemberDescriptionFactory$GeneralMemberDescriptionFactory::create_member_descriptions::h91c371565c4be6372073d67a738909bfd1e67a95a797ecd3b33fb85a876ed718s3av::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#157 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#158 0x00007ffff45743ce in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#159 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#160 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#161 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#162 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#163 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#164 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#165 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#166 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#167 0x00007ffff457454c in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#168 0x00007ffff45806d5 in middle::trans::debuginfo::subroutine_type_metadata::ha5a1676df1bf03124dab60afe77c8b2175f365a1a2283e1983706fe1480ede42aB::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#169 0x00007ffff45745f8 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#170 0x00007ffff457c4c5 in middle::trans::debuginfo::MemberDescriptionFactory$EnumVariantMemberDescriptionFactory::create_member_descriptions::h2e78253ab000ae1701de1bfbbaffa0843f2ff76e9c1bfd68026762b7579d0b73H7aa::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#171 0x00007ffff457ae66 in middle::trans::debuginfo::MemberDescriptionFactory$GeneralMemberDescriptionFactory::create_member_descriptions::h91c371565c4be6372073d67a738909bfd1e67a95a797ecd3b33fb85a876ed718s3av::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#172 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#173 0x00007ffff45743ce in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#174 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#175 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#176 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#177 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#178 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#179 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#180 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#181 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#182 0x00007ffff457454c in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#183 0x00007ffff45806d5 in middle::trans::debuginfo::subroutine_type_metadata::ha5a1676df1bf03124dab60afe77c8b2175f365a1a2283e1983706fe1480ede42aB::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
---Type <return> to continue, or q <return> to quit---
#184 0x00007ffff45745f8 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#185 0x00007ffff457c4c5 in middle::trans::debuginfo::MemberDescriptionFactory$EnumVariantMemberDescriptionFactory::create_member_descriptions::h2e78253ab000ae1701de1bfbbaffa0843f2ff76e9c1bfd68026762b7579d0b73H7aa::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#186 0x00007ffff457ae66 in middle::trans::debuginfo::MemberDescriptionFactory$GeneralMemberDescriptionFactory::create_member_descriptions::h91c371565c4be6372073d67a738909bfd1e67a95a797ecd3b33fb85a876ed718s3av::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#187 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#188 0x00007ffff45743ce in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#189 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#190 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#191 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#192 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#193 0x00007ffff4578832 in middle::trans::debuginfo::__extensions__::create_member_descriptions::anon::expr_fn::aK () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#194 0x00007ffff4577f83 in middle::trans::debuginfo::MemberDescriptionFactory$StructMemberDescriptionFactory::create_member_descriptions::he69e430887431e09eddfdab7d6d505afbfd4b160d9ebf618f0062091289a2596pmat::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#195 0x00007ffff45795a9 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::hd761a4a11c7a683d000cb48a6fa0cdf9b9123a59aff2475cf159911a130e01abiQa8::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#196 0x00007ffff4574b43 in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#197 0x00007ffff457454c in middle::trans::debuginfo::type_metadata::hebcc6ebb94d2cf6dea85484a0919c1f58cf987eba101eceaf65315c5005d5b7ca3::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#198 0x00007ffff45707ea in middle::trans::debuginfo::declare_local::h7375a9b2222eceff009f515099c0f540f2fc543030da7eeb015215762b458ff8an::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#199 0x00007ffff456fe16 in middle::trans::debuginfo::create_local_var_metadata::anon::expr_fn::ap () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#200 0x00007ffff4869feb in middle::pat_util::pat_bindings::anon::expr_fn::ay () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#201 0x00007ffff6900a41 in ast_util::walk_pat::h492c86586a516b5980ad5369169ebc7beb82d3d68c99275918873633a4fd2311av::v0.9.pre () from /usr/local/bin/../lib/libsyntax-2e4c0458-0.9-pre.so
#202 0x00007ffff45423a8 in middle::pat_util::pat_bindings::h87c87c5fed5d625f818591f5f863e9c936216d4b0135239e5dd99e0c47e2870bax::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#203 0x00007ffff44f6117 in middle::trans::debuginfo::create_local_var_metadata::hc103864328d94677504cb8e32debec783544a39ec2334db0f677370c8f8f8429ai::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#204 0x00007ffff4406b1c in middle::trans::base::trans_stmt::ha8bbe08636deb51e011d17de83918c2b63272da1f8a10f8c8846a0eb252d0d6eaN::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#205 0x00007ffff44051a2 in middle::trans::controlflow::trans_block::hd1e62303ca78e746d61c01a4c6ee68b4f6c6502faf053847cb7ec483291c54f0aq::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#206 0x00007ffff44767dd in middle::trans::expr::trans_rvalue_dps_unadjusted::anon::expr_fn::ae () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#207 0x00007ffff4413652 in middle::trans::base::with_scope::h3780c9247872e2b9969bbf6b4bc9e88eaeb0d15a4c26514dc86669771b599561ah::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#208 0x00007ffff446e9e1 in middle::trans::expr::trans_rvalue_dps_unadjusted::h365f789c599dae3f157ffe5d44eb519d65b7da53e7898a7c271193f924f533d6a9::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#209 0x00007ffff44075f3 in middle::trans::expr::trans_into::hf84e667efc7300d274897302637780ccd9da102af822492c568281fe7ed876ebaX::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#210 0x00007ffff4405242 in middle::trans::controlflow::trans_block::hd1e62303ca78e746d61c01a4c6ee68b4f6c6502faf053847cb7ec483291c54f0aq::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#211 0x00007ffff4502520 in middle::trans::base::trans_closure::h162d3f701e0536738be80c4646ddd14cb3042522d2c12666968c48c991931903aT::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#212 0x00007ffff43d3e0a in middle::trans::base::trans_fn::h199d356ce8c61121ace3a9f6626d3c21005820128d155dd942d6c3103df6fe9aaw::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#213 0x00007ffff4505067 in middle::trans::foreign::trans_rust_fn_with_foreign_abi::h22b965bd8178204728a0fe4fe17c62e3b859697e9efe66cc54f15ebd3a70d469al::v0.9.pre ()
   from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#214 0x00007ffff43cb433 in middle::trans::base::trans_item::h95d24e79b84fc2e7eaf4d392b4b5cdb92181441b6723570167f14029bc355fa1a1::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#215 0x00007ffff4507f3f in middle::trans::base::trans_mod::h9a271200d217dca853faed8f899c8eb70b5cefc34993c091b01016442460c924aq::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#216 0x00007ffff43caef0 in middle::trans::base::trans_item::h95d24e79b84fc2e7eaf4d392b4b5cdb92181441b6723570167f14029bc355fa1a1::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#217 0x00007ffff4507f3f in middle::trans::base::trans_mod::h9a271200d217dca853faed8f899c8eb70b5cefc34993c091b01016442460c924aq::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#218 0x00007ffff4517bcf in middle::trans::base::trans_crate::h8b40392bd768ac99b8925e8b83da6709c679f357248c265bb90a8ce65f94d3a3ac::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#219 0x00007ffff4a7aa5e in driver::driver::phase_4_translate_to_llvm::h6ad8669a37cf0ce519a28b3d3adba748024d04cc4417f6296825d6d998a13b4eae::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#220 0x00007ffff4a7e245 in driver::driver::compile_input::h0d1339743aa7a569106f648bc988fff9469c3c19fc9aab385fee64a15b4773eaaz::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#221 0x00007ffff4aa24dc in run_compiler::h8e8a0a538841daa75ab2cb364a5700c9e22d66a8e23b1f5c81d294ef694d902fav::v0.9.pre () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#222 0x00007ffff4abe0fd in main_args::anon::expr_fn::ah () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#223 0x00007ffff4abc0c4 in monitor::anon::expr_fn::ay () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#224 0x00007ffff4ab78a5 in task::TaskBuilder::try::anon::expr_fn::LIaxaa () from /usr/local/bin/../lib/librustc-5b94a16f-0.9-pre.so
#225 0x00007ffff7938b38 in rt::task::__extensions__::build_start_wrapper::anon::anon::expr_fn::ai () from /usr/local/bin/../lib/libstd-04ff901e-0.9-pre.so
#226 0x00007ffff7937303 in rt::task::__extensions__::run::anon::expr_fn::a4 () from /usr/local/bin/../lib/libstd-04ff901e-0.9-pre.so
#227 0x00007ffff7951333 in rust_try (f=<optimized out>, fptr=<optimized out>, env=<optimized out>) at /home/jdm/sdb/rust/src/rt/rust_cxx_glue.cpp:20
#228 0x00007ffff7937213 in rt::task::Unwinder::try::h061ae80bc4171ef466bdd4dc31dbbdadafc4fc075a7671718af8f0a320ee240cfea3::v0.9.pre () from /usr/local/bin/../lib/libstd-04ff901e-0.9-pre.so
#229 0x00007ffff7936e5f in rt::task::Task::run::h8144bca88dba2788c3e3849daa4f007009cd4f41197c25e5cce634da3be546ad3Sa1::v0.9.pre () from /usr/local/bin/../lib/libstd-04ff901e-0.9-pre.so
---Type <return> to continue, or q <return> to quit---
#230 0x00007ffff7938174 in rt::task::__extensions__::build_start_wrapper::anon::expr_fn::aY () from /usr/local/bin/../lib/libstd-04ff901e-0.9-pre.so
#231 0x0000000000000000 in ?? ()
