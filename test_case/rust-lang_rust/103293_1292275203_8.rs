Rust
    if assets_symbol.exists() {
        fs::remove_dir(assets_symbol.clone())
            .expect(&format!("失败：不能删除原来的 {} 符号链接文件", assets_symbol.display())[..]);
    }
