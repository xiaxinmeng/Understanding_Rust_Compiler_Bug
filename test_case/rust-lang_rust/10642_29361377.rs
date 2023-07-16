 diff
diff --git a/src/etc/vim/syntax/rust.vim b/src/etc/vim/syntax/rust.vim
index e5ff089..dc1e58c 100644
--- a/src/etc/vim/syntax/rust.vim
+++ b/src/etc/vim/syntax/rust.vim
@@ -187,8 +187,8 @@ syn match   rustCharacter   /'\([^'\\]\|\\\([nrt0\\'"]\|x\x\{2}\|u\x\{4}\|U\x\{8

 syn region    rustCommentML   start="/\*" end="\*/" contains=rustTodo
 syn region    rustComment     start="//" end="$" contains=rustTodo keepend
-syn region    rustCommentMLDoc start="/\*\%(!\|\*/\@!\)" end="\*/" contains=rustTodo
-syn region    rustCommentDoc  start="//[/!]" end="$" contains=rustTodo keepend
+syn region    rustCommentMLDoc start="/\*\%(!\|\*[*/]\@!\)" end="\*/" contains=rustTodo
+syn region    rustCommentDoc  start="//\%(//\@!\|!\)" end="$" contains=rustTodo keepend

 syn keyword rustTodo contained TODO FIXME XXX NB NOTE
