 diff
diff --git a/src/etc/vim/ftplugin/rust.vim b/src/etc/vim/ftplugin/rust.vim
index 281a63e..dcd4f44 100644
--- a/src/etc/vim/ftplugin/rust.vim
+++ b/src/etc/vim/ftplugin/rust.vim
@@ -42,4 +42,55 @@ if exists("g:loaded_delimitMate")
    let b:delimitMate_excluded_regions = delimitMate#Get("excluded_regions") . ',rustLifetimeCandidate,rustGenericLifetimeCandidate'
 endif

-let b:undo_ftplugin = "setlocal formatoptions< comments< commentstring< includeexpr< suffixesadd< | if exists('b:rust_original_delimitMate_excluded_regions') | let b:delimitMate_excluded_regions = b:rust_original_delimitMate_excluded_regions | unlet b:rust_original_delimitMate_excluded_regions | elseif exists('b:delimitMate_excluded_regions') | unlet b:delimitMate_excluded_regions | endif"
+" Bind motion commands to support hanging indents
+nnoremap <silent> <buffer> [[ :call <SID>Rust_Jump('n', 'Back')<CR>
+nnoremap <silent> <buffer> ]] :call <SID>Rust_Jump('n', 'Forward')<CR>
+xnoremap <silent> <buffer> [[ :call <SID>Rust_Jump('v', 'Back')<CR>
+xnoremap <silent> <buffer> ]] :call <SID>Rust_Jump('v', 'Forward')<CR>
+onoremap <silent> <buffer> [[ :call <SID>Rust_Jump('o', 'Back')<CR>
+onoremap <silent> <buffer> ]] :call <SID>Rust_Jump('o', 'Forward')<CR>
+
+let b:undo_ftplugin = "
+       \setlocal formatoptions< comments< commentstring< includeexpr< suffixesadd<
+       \|if exists('b:rust_original_delimitMate_excluded_regions')
+         \|let b:delimitMate_excluded_regions = b:rust_original_delimitMate_excluded_regions
+         \|unlet b:rust_original_delimitMate_excluded_regions
+       \|elseif exists('b:delimitMate_excluded_regions')
+         \|unlet b:delimitMate_excluded_regions
+       \|endif
+       \|nunmap <buffer> [[
+       \|nunmap <buffer> ]]
+       \|xunmap <buffer> [[
+       \|xunmap <buffer> ]]
+       \|ounmap <buffer> [[
+       \|ounmap <buffer> ]]
+       \"
+
+if exists('*<SID>Rust_Jump') | finish | endif
+
+function! <SID>Rust_Jump(mode, function) range
+   let cnt = v:count1
+   normal! m'
+   if a:mode ==# 'v'
+       norm! gv
+   endif
+   let foldenable = &foldenable
+   set nofoldenable
+   while cnt > 0
+       execute "call <SID>Rust_Jump_" . a:function . "()"
+       let cnt = cnt - 1
+   endwhile
+   let &foldenable = foldenable
+endfunction
+
+function! <SID>Rust_Jump_Back()
+   call search('{', 'b')
+   keepjumps normal! w99[{
+endfunction
+
+function! <SID>Rust_Jump_Forward()
+   normal! j0
+   call search('{', 'b')
+   keepjumps normal! w99[{%
+   call search('{')
+endfunction
