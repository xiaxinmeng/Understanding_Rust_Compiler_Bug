xml
 <Type Name="slice&lt;*&gt;">
    <DisplayString>{{ length={length} }}</DisplayString>
    <Expand>
      <Item Name="[size]" ExcludeView="simple">length</Item>
      <ArrayItems>
        <Size>length</Size>
        <ValuePointer>data_ptr</ValuePointer>
      </ArrayItems>
    </Expand>
    </Type>
    <!-- added -->
    <Type Name="mut slice&lt;*&gt;">
    <DisplayString>{{ length={length} }}</DisplayString>
    <Expand>
      <Item Name="[size]" ExcludeView="simple">length</Item>
      <ArrayItems>
        <Size>length</Size>
        <ValuePointer>data_ptr</ValuePointer>
      </ArrayItems>
    </Expand>
  </Type>
