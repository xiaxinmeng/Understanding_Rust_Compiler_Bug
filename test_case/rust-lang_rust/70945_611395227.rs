rust
_1 = SomeStruct;
_2 = &_1;
_3 = _2 as &dyn SomeTrait;
<dyn Some Trait as SomeTrait>::some_method(_3);
