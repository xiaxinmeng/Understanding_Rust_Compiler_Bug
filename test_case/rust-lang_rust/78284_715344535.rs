
       --proto <protocols>
              Tells curl to limit what protocols it may use in the  transfer.  Protocols  are
              evaluated  left  to right, are comma separated, and are each a protocol name or
              'all', optionally prefixed by zero or more modifiers. Available modifiers are:

              +  Permit this protocol in addition to protocols already permitted (this is the
                 default if no modifier is used).

              -  Deny  this  protocol, removing it from the list of protocols already permit‐
                 ted.

              =  Permit only this protocol (ignoring the list already permitted), though sub‐
                 ject  to  later  modification  by  subsequent entries in the comma separated
                 list.
