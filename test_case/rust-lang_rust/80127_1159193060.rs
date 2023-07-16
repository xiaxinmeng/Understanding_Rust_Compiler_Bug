diff
PassMode::Indirect { ref attrs, extra_attrs: None, on_stack: true } => {
    let i = apply(attrs);
    let byval = llvm::CreateByValAttr(cx.llcx, arg.layout.llvm_type(cx));
-   attributes::apply_to_llfn(llfn, llvm::AttributePlace::Argument(i), &[byval]);
+   let align = llvm::CreateAlignmentAttr(cx.llcx, arg.layout.align().abi);
+   attributes::apply_to_llfn(llfn, llvm::AttributePlace::Argument(i), &[byval, align]);
}
