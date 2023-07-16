xml
  <Type Name="sage_core::constraint::Constraint">
    <DisplayString Condition="RUST$ENUM$DISR == 0x0">{{ Binary }}</DisplayString>
    <DisplayString Condition="RUST$ENUM$DISR == 0x1">{{ Unary }}</DisplayString>
    <Expand>
      <Item Name="op" Condition="RUST$ENUM$DISR == 0x0">*(sage_core::constraint::BinaryOp*)(((char*)&amp;RUST$ENUM$DISR) + 1)</Item>
      <Item Name="left" Condition="RUST$ENUM$DISR == 0x0">*(alloc::rc::Rc&lt;sage_core::constraint::Constraint&gt;*)(((char*)&amp;RUST$ENUM$DISR) + sizeof(void*))</Item>
      <Item Name="right" Condition="RUST$ENUM$DISR == 0x0">*(alloc::rc::Rc&lt;sage_core::constraint::Constraint&gt;*)(((char*)&amp;RUST$ENUM$DISR) + 2*sizeof(void*))</Item>
      <Item Name="op" Condition="RUST$ENUM$DISR == 0x1">*(sage_core::constraint::UnaryOp*)(((char*)&amp;RUST$ENUM$DISR) + 1)</Item>
      <Item Name="child" Condition="RUST$ENUM$DISR == 0x1">*(alloc::rc::Rc&lt;sage_core::constraint::Constraint&gt;*)(((char*)&amp;RUST$ENUM$DISR) + sizeof(void*))</Item>
    </Expand>
  </Type>
