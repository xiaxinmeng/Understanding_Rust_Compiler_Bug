

;; Function test (_Z4testv, funcdef_no=0, decl_uid=2368, cgraph_uid=1, symbol_order=0)

int test ()
{
  int n;
  unsigned int n.0_1;
  unsigned int _2;
  int _3;
  int _6;

  <bb 2> :
  # DEBUG BEGIN_STMT
  # DEBUG n => 113383
  # DEBUG BEGIN_STMT
  goto <bb 6>; [INV]

  <bb 3> :
  # DEBUG BEGIN_STMT
  n.0_1 = (unsigned int) n_4;
  _2 = n.0_1 & 1;
  if (_2 == 0)
    goto <bb 4>; [INV]
  else
    goto <bb 5>; [INV]

  <bb 4> :
  # DEBUG BEGIN_STMT
  n_9 = n_4 / 2;
  # DEBUG n => n_9
  goto <bb 6>; [INV]

  <bb 5> :
  # DEBUG BEGIN_STMT
  _3 = n_4 * 3;
  n_8 = _3 + 1;
  # DEBUG n => n_8

  <bb 6> :
  # n_4 = PHI <113383(2), n_9(4), n_8(5)>
  # DEBUG n => n_4
  # DEBUG BEGIN_STMT
  if (n_4 != 1)
    goto <bb 3>; [INV]
  else
    goto <bb 7>; [INV]

  <bb 7> :
  # DEBUG BEGIN_STMT
  _6 = n_4;
  return _6;

}



;; Function test_1 (_Z6test_1v, funcdef_no=1, decl_uid=2376, cgraph_uid=2, symbol_order=1)

int test_1 ()
{
  int D.2391;

  <bb 2> :
  # DEBUG BEGIN_STMT
  # DEBUG n => 113383
  # DEBUG n => NULL
  return 1;

}
