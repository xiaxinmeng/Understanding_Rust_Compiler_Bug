diff
405c405
<         atomic::fence(Acquire);
---
>         this.inner().strong.load(Acquire);
742c742
<             atomic::fence(Acquire);
---
>             self.inner().weak.load(Acquire);
1246c1246
<         atomic::fence(Acquire);
---
>         self.inner().strong.load(Acquire);
1704c1704
<             atomic::fence(Acquire);
---
>             inner.weak.load(Acquire);
