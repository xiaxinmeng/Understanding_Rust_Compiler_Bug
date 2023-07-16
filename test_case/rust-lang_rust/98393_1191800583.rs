xml
  <Type Name="enum$&lt;*&gt;::Variant0">
    <AlternativeType Name="enum$&lt;*&gt;::Variant1" />
    <AlternativeType Name="enum$&lt;*&gt;::Variant2" />
    ...
    <Intrinsic Name="begin_lo" Expression="DISCR128_BEGIN_LO" Optional="true" />
    <Intrinsic Name="begin_lo" Expression="DISCR128_EXACT_LO" Optional="true" />
    <Intrinsic Name="begin_lo" Expression="DISCR_BEGIN" Optional="true" />
    <Intrinsic Name="begin_lo" Expression="DISCR_EXACT" Optional="true" />

    <Intrinsic Name="begin_hi" Expression="DISCR128_BEGIN_HI" Optional="true" />
    <Intrinsic Name="begin_hi" Expression="DISCR128_EXACT_HI" Optional="true" />
    <Intrinsic Name="begin_hi" Expression="0" Optional="true" />
    <Intrinsic Name="begin_hi" Expression="0" Optional="true" />

    (same for end_hi/lo)
  </Type>
...
  <Type Name="enum$&lt;*&gt;">
      <DisplayString Condition="in_range(variant0.begin_hi(), variant0.begin_lo(), variant0.end_hi(), variant0.end_lo()"> ...</DisplayString>
      <DisplayString Condition="in_range(variant1.begin_hi(), variant1.begin_lo(), variant1.end_hi(), variant1.end_lo()"> ...</DisplayString>
      ...
  </Type>
