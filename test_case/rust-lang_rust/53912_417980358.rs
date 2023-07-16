cxx
  /// Convenience method for creating a promoted global name
  /// for the given value name of a local, and its original module's ID.
  static std::string getGlobalNameForLocal(StringRef Name, ModuleHash ModHash) {
    SmallString<256> NewName(Name);
    NewName += ".llvm.";
    NewName += utostr((uint64_t(ModHash[0]) << 32) |
                      ModHash[1]); // Take the first 64 bits
    return NewName.str();
  }

  /// Helper to obtain the unpromoted name for a global value (or the original
  /// name if not promoted).
  static StringRef getOriginalNameBeforePromote(StringRef Name) {
    std::pair<StringRef, StringRef> Pair = Name.split(".llvm.");
    return Pair.first;
  }
