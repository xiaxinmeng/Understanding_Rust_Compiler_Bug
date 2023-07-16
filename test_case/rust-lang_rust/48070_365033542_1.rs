
A --+--> `TMP1 = &mut foo` (B1) ---------------------------------+-> `emit(TMP1)`
       |                                                         ^        ^
       +--> `TMP2 = &mut foo` (B2)` --> `TMP1 = twaddle(TMP2)` --+        |
                                                         ^                |
                                                         |                |
        Borrow B2 activates here, at first use of TMP2 --+                |
                                                                          |
                Borrow B1 activates here, at first use of TMP1 -----------+
