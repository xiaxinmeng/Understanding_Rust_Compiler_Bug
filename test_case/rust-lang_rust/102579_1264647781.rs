plain
   Compiling toml v0.5.9
error[E0424]: expected value, found module `self`
    --> dist.rs:1860:37
     |
1846 | fn maybe_install_llvm(builder: &Builder<'_>, target: TargetSelection, dst_libdir: &Path) -> bool {
     |    ------------------ this function can't have a `self` parameter
...
1860 |     let should_install_llvm = match self.config.target_config.get(&target) {
     |                                     ^^^^ `self` value is a keyword only available in methods with a `self` parameter
error[E0422]: cannot find struct, variant or union type `Target` in this scope
    --> dist.rs:1863:14
     |
     |
1863 |         Some(Target { llvm_config: Some(_), .. }) => self.config.llvm_from_ci && target == self.config.build,
     |
help: consider importing one of these items
     |
11   | use crate::DependencyType::Target;
11   | use crate::DependencyType::Target;
     |
11   | use crate::Target;
     |

error[E0424]: expected value, found module `self`
    --> dist.rs:1863:54
     |
1846 | fn maybe_install_llvm(builder: &Builder<'_>, target: TargetSelection, dst_libdir: &Path) -> bool {
     |    ------------------ this function can't have a `self` parameter
...
1863 |         Some(Target { llvm_config: Some(_), .. }) => self.config.llvm_from_ci && target == self.config.build,
     |                                                      ^^^^ `self` value is a keyword only available in methods with a `self` parameter
error[E0424]: expected value, found module `self`
    --> dist.rs:1863:92
     |
     |
1846 | fn maybe_install_llvm(builder: &Builder<'_>, target: TargetSelection, dst_libdir: &Path) -> bool {
     |    ------------------ this function can't have a `self` parameter
...
1863 |         Some(Target { llvm_config: Some(_), .. }) => self.config.llvm_from_ci && target == self.config.build,
     |                                                                                            ^^^^ `self` value is a keyword only available in methods with a `self` parameter
error[E0422]: cannot find struct, variant or union type `Target` in this scope
    --> dist.rs:1864:14
     |
     |
1864 |         Some(Target { llvm_config: None, .. }) | None => true,
     |
help: consider importing one of these items
     |
11   | use crate::DependencyType::Target;
