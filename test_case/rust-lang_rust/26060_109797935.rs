 bash
(master)funkill@fennec ~/work/self/rust/rust $ diff ./src/rustbook/ ../rustbook/src/
(master)funkill@fennec ~/work/self/rust/rust $ git checkout fix_rustbook 
Переключено на ветку «fix_rustbook»
(fix_rustbook)funkill@fennec ~/work/self/rust/rust $ diff ./src/rustbook/ ../rustbook/src/
diff ./src/rustbook/build.rs ../rustbook/src/build.rs
62c62
<                  current_page.path_to_root.join(&item.path).with_extension("html").display(),
---
>                  item.path_to_root.join(&item.path.with_extension("html")).display(),
(fix_rustbook)funkill@fennec ~/work/self/rust/rust $ cd ../rustbook
(master)funkill@fennec ~/work/self/rust/rustbook $ git remote show origin |grep URL                                                                          
2:  URL для извлечения: https://github.com/steveklabnik/rustbook
3:  URL для отправки: https://github.com/steveklabnik/rustbook
(master)funkill@fennec ~/work/self/rust/rustbook $ git pull origin master
Из https://github.com/steveklabnik/rustbook
 * branch            master     -> FETCH_HEAD
Already up-to-date.
(master)funkill@fennec ~/work/self/rust/rustbook $ cargo run -- build ../rust_book_ru
   Compiling rustbook v0.3.0 (file:///home/funkill/work/self/rust/rustbook)
     Running `target/debug/rustbook build ../rust_book_ru`
(master)funkill@fennec ~/work/self/rust/rustbook $ grep -m 4 -E "^<li><a [a-z\=\']* href=" ./_book/README.html                                               
31:<li><a class='active' href='./README.html'><b>1.</b> Introduction</a>
33:<li><a  href='../src/INTRODUCTION.html'><b>2.</b> Введение</a>
35:<li><a  href='../src/getting-started.html'><b>3.</b> C чего начать</a>
37:<li><a  href='../src/installing-rust.html'><b>3.1.</b> Установка Rust</a>
(master)funkill@fennec ~/work/self/rust/rustbook $ grep -m 4 -E "^<li><a [a-z\=\']* href=" ./_book/src/INTRODUCTION.html                                     
31:<li><a  href='./README.html'><b>1.</b> Introduction</a>
33:<li><a class='active' href='../src/INTRODUCTION.html'><b>2.</b> Введение</a>
35:<li><a  href='../src/getting-started.html'><b>3.</b> C чего начать</a>
37:<li><a  href='../src/installing-rust.html'><b>3.1.</b> Установка Rust</a>
(master)funkill@fennec ~/work/self/rust/rustbook $ rm -rf ./_book
(master)funkill@fennec ~/work/self/rust/rustbook $ cargo run -- build ../rust/src/doc/trpl
     Running `target/debug/rustbook build ../rust/src/doc/trpl`
(master)funkill@fennec ~/work/self/rust/rustbook $ grep -m 4 -E "^<li><a [a-z\=\']* href=" ./_book/README.html                 
31:<li><a class='active' href='./README.html'><b>1.</b> Introduction</a>
33:<li><a  href='getting-started.html'><b>2.</b> Getting Started</a>
35:<li><a  href='installing-rust.html'><b>2.1.</b> Installing Rust</a>
37:<li><a  href='hello-world.html'><b>2.2.</b> Hello, world!</a>
(master)funkill@fennec ~/work/self/rust/rustbook $ grep -m 4 -E "^<li><a [a-z\=\']* href=" ./_book/getting-started.html 
31:<li><a  href='./README.html'><b>1.</b> Introduction</a>
33:<li><a class='active' href='getting-started.html'><b>2.</b> Getting Started</a>
35:<li><a  href='installing-rust.html'><b>2.1.</b> Installing Rust</a>
37:<li><a  href='hello-world.html'><b>2.2.</b> Hello, world!</a>
(master)funkill@fennec ~/work/self/rust/rustbook $ cp -v ../rust/src/rustbook/* ./src/                                                                       
«../rust/src/rustbook/book.rs» -> «./src/book.rs»
«../rust/src/rustbook/build.rs» -> «./src/build.rs»
«../rust/src/rustbook/css.rs» -> «./src/css.rs»
«../rust/src/rustbook/error.rs» -> «./src/error.rs»
«../rust/src/rustbook/help.rs» -> «./src/help.rs»
«../rust/src/rustbook/javascript.rs» -> «./src/javascript.rs»
«../rust/src/rustbook/main.rs» -> «./src/main.rs»
«../rust/src/rustbook/serve.rs» -> «./src/serve.rs»
«../rust/src/rustbook/subcommand.rs» -> «./src/subcommand.rs»
«../rust/src/rustbook/term.rs» -> «./src/term.rs»
«../rust/src/rustbook/test.rs» -> «./src/test.rs»
(master)funkill@fennec ~/work/self/rust/rustbook $ rm -rf ./_book
(master)funkill@fennec ~/work/self/rust/rustbook $ git status
На ветке master
Ваша ветка обновлена в соответствии с «origin/master».
Изменения, которые не в индексе для коммита:
  (используйте «git add <файл>…», чтобы добавить файл в индекс)
  (используйте «git checkout -- <файл>…», чтобы отменить изменения
   в рабочем каталоге)

        изменено:      src/build.rs

нет изменений добавленных для коммита
(используйте «git add» и/или «git commit -a»)
(master)funkill@fennec ~/work/self/rust/rustbook $ git diff ./src/build.rs
diff --git a/src/build.rs b/src/build.rs
index 31c9732..5ffb9b0 100755
--- a/src/build.rs
+++ b/src/build.rs
@@ -59,7 +59,7 @@ fn write_toc(book: &Book, current_page: &BookItem, out: &mut Write) -> io::Resul

         try!(writeln!(out, "<li><a {} href='{}'><b>{}</b> {}</a>",
                  class_string,
-                 item.path_to_root.join(&item.path.with_extension("html")).display(),
+                 current_page.path_to_root.join(&item.path).with_extension("html").display(),
                  section,
                  item.title));
         if !item.children.is_empty() {
(master)funkill@fennec ~/work/self/rust/rustbook $ cargo run -- build ../rust_book_ru                                                                        
   Compiling rustbook v0.3.0 (file:///home/funkill/work/self/rust/rustbook)
     Running `target/debug/rustbook build ../rust_book_ru`
(master)funkill@fennec ~/work/self/rust/rustbook $ grep -m 4 -E "^<li><a [a-z\=\']* href=" ./_book/README.html 
31:<li><a class='active' href='./README.html'><b>1.</b> Introduction</a>
33:<li><a  href='./src/INTRODUCTION.html'><b>2.</b> Введение</a>
35:<li><a  href='./src/getting-started.html'><b>3.</b> C чего начать</a>
37:<li><a  href='./src/installing-rust.html'><b>3.1.</b> Установка Rust</a>
(master)funkill@fennec ~/work/self/rust/rustbook $ grep -m 4 -E "^<li><a [a-z\=\']* href=" ./_book/src/INTRODUCTION.html 
31:<li><a  href='../README.html'><b>1.</b> Introduction</a>
33:<li><a class='active' href='../src/INTRODUCTION.html'><b>2.</b> Введение</a>
35:<li><a  href='../src/getting-started.html'><b>3.</b> C чего начать</a>
37:<li><a  href='../src/installing-rust.html'><b>3.1.</b> Установка Rust</a>
(master)funkill@fennec ~/work/self/rust/rustbook $ grep -m 4 -E "^<li><a [a-z\=\']* href=" ./_book/README.html             
31:<li><a class='active' href='./README.html'><b>1.</b> Introduction</a>
33:<li><a  href='./getting-started.html'><b>2.</b> Getting Started</a>
35:<li><a  href='./installing-rust.html'><b>2.1.</b> Installing Rust</a>
37:<li><a  href='./hello-world.html'><b>2.2.</b> Hello, world!</a>
(master)funkill@fennec ~/work/self/rust/rustbook $ grep -m 4 -E "^<li><a [a-z\=\']* href=" ./_book/getting-started.html 
31:<li><a  href='README.html'><b>1.</b> Introduction</a>
33:<li><a class='active' href='getting-started.html'><b>2.</b> Getting Started</a>
35:<li><a  href='installing-rust.html'><b>2.1.</b> Installing Rust</a>
37:<li><a  href='hello-world.html'><b>2.2.</b> Hello, world!</a>
