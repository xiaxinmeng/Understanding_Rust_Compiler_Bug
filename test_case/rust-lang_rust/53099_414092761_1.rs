
  if (!CS) {
    // CallSite is zero, fallback to ABI type alignment
    return DL.getABITypeAlignment(Ty);
  }
