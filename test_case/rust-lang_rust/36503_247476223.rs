 xml
<?xml version="1.0" encoding="utf-8"?>

<AutoVisualizer xmlns="http://schemas.microsoft.com/vstudio/debugger/natvis/2010">
  <Type Name="collections::vec::Vec&lt;*&gt;">
    <DisplayString>{{ length={len} }}</DisplayString>
    <Expand>
      <Item Name="[length]" ExcludeView="simple">len</Item>
      <Item Name="[capacity]" ExcludeView="simple">buf.cap</Item>
      <ArrayItems>
        <Size>len</Size>
        <ValuePointer>buf.ptr.pointer.__0</ValuePointer>
      </ArrayItems>
    </Expand>
  </Type>
</AutoVisualizer>
