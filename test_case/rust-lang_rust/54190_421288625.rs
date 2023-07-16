cpp
    // In simple cases, only Name is set. Renamed exports are parsed
    // and set as "ExtName = Name". If Name has the form "OtherDll.Func",
    // it shouldn't be a normal exported function but a forward to another
    // DLL instead. This is supported by both MS and GNU linkers.
    if (E1.ExtName != E1.Name && StringRef(E1.Name).contains('.')) {
      E2.Name = Saver.save(E1.ExtName);
      E2.ForwardTo = Saver.save(E1.Name);
      Config->Exports.push_back(E2);
      continue;
    }
