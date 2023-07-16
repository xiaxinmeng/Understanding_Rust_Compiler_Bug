 rst
Note the documentation for the primitives :primitive:`str` and
:primitive:`[T]` (also called 'slice'). Many method calls on
:struct:`~std::string::String` and :struct:`~std::vec::Vec` are
actually calls to methods on :primitive:`str` and :primitive:`[T]`
respectively, via :book:`deref coercions`.
