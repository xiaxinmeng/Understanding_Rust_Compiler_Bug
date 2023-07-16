c++
// Do not use 'gnu' hash style for Mips targets because .gnu.hash
  // and the MIPS ABI require .dynsym to be sorted in different ways.
  // .gnu.hash needs symbols to be grouped by hash code whereas the MIPS
  // ABI requires a mapping between the GOT and the symbol table.
  // Android loader does not support .gnu.hash until API 23.
  // Hexagon linker/loader does not support .gnu.hash
  if (!IsMips && !IsHexagon) {
    if (Distro.IsRedhat() || Distro.IsOpenSUSE() || Distro.IsAlpineLinux() ||
        (Distro.IsUbuntu() && Distro >= Distro::UbuntuMaverick) ||
        (IsAndroid && !Triple.isAndroidVersionLT(23)))
      ExtraOpts.push_back("--hash-style=gnu");

    if (Distro.IsDebian() || Distro.IsOpenSUSE() ||
        Distro == Distro::UbuntuLucid || Distro == Distro::UbuntuJaunty ||
        Distro == Distro::UbuntuKarmic ||
        (IsAndroid && Triple.isAndroidVersionLT(23)))
      ExtraOpts.push_back("--hash-style=both");
  }
