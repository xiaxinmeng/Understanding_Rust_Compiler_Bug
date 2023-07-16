
if (ST->getProcFamily() != AArch64Subtarget::Others &&
  !ST->getSchedModel().isOutOfOrder()) {
  UP.Runtime = true;
  // ...
}
