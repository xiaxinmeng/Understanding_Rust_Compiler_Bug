plain
status: exit code: 2
command: "make"
stdout:
------------------------------------------
make[1]: Entering directory '/d/a/rust/rust/src/test/run-make-fulldeps/coverage-spanview'
# Compile the test library with coverage instrumentation
PATH="/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview:D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin:/c/Program Files (x86)/Windows Kits/10/bin/x64:/c/Program Files (x86)/Windows Kits/10/bin/10.0.19041.0/x64:/c/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.28.29333/bin/HostX64/x64:/c/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.28.29333/bin/HostX64/x64:/d/a/rust/rust/build/x86_64-pc-windows-msvc/stage0-bootstrap-tools/x86_64-pc-windows-msvc/release/deps:/d/a/rust/rust/build/x86_64-pc-windows-msvc/stage0/bin:/d/a/rust/rust/ninja:/d/a/rust/rust/msys2/mingw64/bin:/c/hostedtoolcache/windows/Python/3.9.1/x64/Scripts:/c/hostedtoolcache/windows/Python/3.9.1/x64:/usr/bin:/d/a/rust/rust/sccache:/c/Users/runneradmin/.dotnet/tools:/c/Program Files/MongoDB/Server/4.4/bin:/c/aliyun-cli:/c/ProgramData/kind:/c/vcpkg:/c/cf-cli:/c/Program Files (x86)/NSIS:/c/Program Files/Mercurial:/c/hostedtoolcache/windows/stack/2.5.1/x64:/c/ProgramData/chocolatey/lib/ghc.8.10.3/tools/ghc-8.10.3/bin:/c/Program Files/dotnet:/c/mysql-5.7.21-winx64/bin:/c/Program Files/R/R-4.0.3/bin/x64:/c/SeleniumWebDrivers/GeckoDriver:/c/Program Files (x86)/sbt/bin:/c/Rust/.cargo/bin:/c/Program Files (x86)/GitHub CLI:/c/Program Files/Git/bin:/c/Program Files (x86)/pipx_bin:/c/hostedtoolcache/windows/go/1.14.14/x64/bin:/c/hostedtoolcache/windows/Python/3.7.9/x64/Scripts:/c/hostedtoolcache/windows/Python/3.7.9/x64:/c/hostedtoolcache/windows/Ruby/2.5.8/x64/bin:/c/Program Files/Java/jdk8u282-b08/bin:/c/npm/prefix:/c/Program Files/Microsoft SDKs/Azure/Azure Dev Spaces CLI:/c/Program Files/Microsoft SDKs/Azure/Azure Dev Spaces CLI:/c/Program Files (x86)/Microsoft SDKs/Azure/CLI2/wbin:/c/windows/system32:/c/windows:/c/windows/System32/Wbem:/c/windows/System32/WindowsPowerShell/v1.0:/c/windows/System32/OpenSSH:/c/ProgramData/Chocolatey/bin:/c/Program Files/Microsoft/Web Platform Installer:/c/Program Files/Docker:/c/Program Files/PowerShell/7:/c/Program Files/dotnet:/c/Program Files/Microsoft SQL Server/130/Tools/Binn:/c/Program Files/Microsoft SQL Server/Client SDK/ODBC/170/Tools/Binn:/c/Program Files (x86)/Windows Kits/10/Windows Performance Toolkit:/c/Program Files (x86)/Microsoft SQL Server/110/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/140/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/150/DTS/Binn:/c/Program Files/nodejs:/c/ProgramData/chocolatey/lib/pulumi/tools/Pulumi/bin:/c/ProgramData/chocolatey/lib/maven/apache-maven-3.6.3/bin:/c/Program Files/Microsoft Service Fabric/bin/Fabric/Fabric.Code:/c/Program Files/Microsoft SDKs/Service Fabric/Tools/ServiceFabricLocalClusterManager:/c/Program Files/OpenSSL/bin:/c/Strawberry/c/bin:/c/Strawberry/perl/site/bin:/c/Strawberry/perl/bin:/c/Program Files/Git/cmd:/c/Program Files/Git/mingw64/bin:/c/Program Files/Git/usr/bin:/c/tools/php:/c/Program Files (x86)/sbt/bin:/c/Program Files/TortoiseSVN/bin:/c/SeleniumWebDrivers/ChromeDriver:/c/SeleniumWebDrivers/EdgeDriver:/c/Program Files/CMake/bin:/c/Program Files/Amazon/AWSCLIV2:/c/Program Files/Amazon/SessionManagerPlugin/bin:/c/Program Files/Amazon/AWSSAMCLI/bin:/c/Program Files (x86)/Google/Cloud SDK/google-cloud-sdk/bin:/c/Program Files (x86)/Microsoft BizTalk Server:/c/Users/runneradmin/AppData/Local/Microsoft/WindowsApps" 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe' --out-dir /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/lib/doctest_crate.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/lib/doctest_crate.rs && echo "--edition=2018" ) \
  --crate-type rlib \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.doctest_crate
for path in "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.doctest_crate/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("C:\hostedtoolcache\windows\Python\3.9.1\x64\python3.exe" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "doctest_crate" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest_crate
rm -f "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest_crate/*
cp "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.doctest_crate/*InstrumentCoverage.0.html "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest_crate/
diff -u --strip-trailing-cr -r expected_mir_dump.doctest_crate/ "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest_crate/
diff -u --strip-trailing-cr -r expected_mir_dump.doctest_crate/doctest_crate.fn_run_in_doctests.-------.InstrumentCoverage.0.html /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.doctest_crate/doctest_crate.fn_run_in_doctests.-------.InstrumentCoverage.0.html
--- expected_mir_dump.doctest_crate/doctest_crate.fn_run_in_doctests.-------.InstrumentCoverage.0.html 2021-01-29 20:40:04.838978600 +0000
+++ /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.doctest_crate/doctest_crate.fn_run_in_doctests.-------.InstrumentCoverage.0.html 2021-01-29 21:40:27.115934200 +0000
@@ -84,16 +84,16 @@
 4:14-4:30: @6[28]: _28 = (_23.0: &amp;&amp;i32)
 4:14-4:30: @6[30]: _29 = (_23.1: &amp;&amp;i32)
 4:14-4:30: @6[33]: _31 = &amp;(*_28)
-4:14-4:30: @6[35]: _32 = &lt;&amp;i32 as Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r &amp;i32, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
-4:14-4:30: @6.Call: _30 = ArgumentV1::new::&lt;&amp;i32&gt;(move _31, move _32) -&gt; [return: bb8, unwind: bb29]
+4:14-4:30: @6[35]: _32 = &lt;&amp;i32 as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r &amp;i32, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
+4:14-4:30: @6.Call: _30 = std::fmt::ArgumentV1::new::&lt;&amp;i32&gt;(move _31, move _32) -&gt; [return: bb8, unwind: bb29]
 4:14-4:30: @8[4]: _34 = &amp;(*_29)
-4:14-4:30: @8[6]: _35 = &lt;&amp;i32 as Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r &amp;i32, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
-4:14-4:30: @8.Call: _33 = ArgumentV1::new::&lt;&amp;i32&gt;(move _34, move _35) -&gt; [return: bb9, unwind: bb29]
+4:14-4:30: @8[6]: _35 = &lt;&amp;i32 as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r &amp;i32, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
+4:14-4:30: @8.Call: _33 = std::fmt::ArgumentV1::new::&lt;&amp;i32&gt;(move _34, move _35) -&gt; [return: bb9, unwind: bb29]
 4:14-4:30: @9[2]: _22 = [move _30, move _33]
 4:14-4:30: @9[7]: _21 = &amp;_22
 4:14-4:30: @9[8]: _20 = &amp;(*_21)
 4:14-4:30: @9[9]: _19 = move _20 as &amp;[std::fmt::ArgumentV1] (Pointer(Unsize))
-4:14-4:30: @9.Call: _14 = Arguments::new_v1(move _15, move _19) -&gt; [return: bb10, unwind: bb29]
+4:14-4:30: @9.Call: _14 = std::fmt::Arguments::new_v1(move _15, move _19) -&gt; [return: bb10, unwind: bb29]
 4:14-4:30: @10.Call: core::panicking::panic_fmt(move _14) -&gt; bb29"><span class="annotation">@6,8,9,10⦊</span>assert_eq!(1, 1)<span class="annotation">⦉@6,8,9,10</span></span><span><span class="code odd" style="--layer: 1" title="4:14-4:30: @7[0]: _0 = const ()"><span class="annotation">⦉@7</span></span></span><span class="code" style="--layer: 0">, // this is run,</span></span>
 <span class="line"><span class="code" style="--layer: 0">        2 =&gt; </span><span><span class="code even" style="--layer: 1" title="5:14-5:30: @13[0]: _0 = const ()"><span class="annotation">@13⦊</span></span></span><span class="code even" style="--layer: 2" title="5:14-5:30: @12[5]: _141 = const fn_run_in_doctests::promoted[3]
 5:14-5:30: @12[6]: _51 = &amp;(*_141)
@@ -108,16 +108,16 @@
 5:14-5:30: @12[28]: _62 = (_57.0: &amp;&amp;i32)
 5:14-5:30: @12[30]: _63 = (_57.1: &amp;&amp;i32)
 5:14-5:30: @12[33]: _65 = &amp;(*_62)
-5:14-5:30: @12[35]: _66 = &lt;&amp;i32 as Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r &amp;i32, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
-5:14-5:30: @12.Call: _64 = ArgumentV1::new::&lt;&amp;i32&gt;(move _65, move _66) -&gt; [return: bb14, unwind: bb29]
+5:14-5:30: @12[35]: _66 = &lt;&amp;i32 as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r &amp;i32, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
+5:14-5:30: @12.Call: _64 = std::fmt::ArgumentV1::new::&lt;&amp;i32&gt;(move _65, move _66) -&gt; [return: bb14, unwind: bb29]
 5:14-5:30: @14[4]: _68 = &amp;(*_63)
-5:14-5:30: @14[6]: _69 = &lt;&amp;i32 as Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r &amp;i32, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
-5:14-5:30: @14.Call: _67 = ArgumentV1::new::&lt;&amp;i32&gt;(move _68, move _69) -&gt; [return: bb15, unwind: bb29]
+5:14-5:30: @14[6]: _69 = &lt;&amp;i32 as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r &amp;i32, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
+5:14-5:30: @14.Call: _67 = std::fmt::ArgumentV1::new::&lt;&amp;i32&gt;(move _68, move _69) -&gt; [return: bb15, unwind: bb29]
 5:14-5:30: @15[2]: _56 = [move _64, move _67]
 5:14-5:30: @15[7]: _55 = &amp;_56
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
 5:14-5:30: @15[8]: _54 = &amp;(*_55)
 5:14-5:30: @15[9]: _53 = move _54 as &amp;[std::fmt::ArgumentV1] (Pointer(Unsize))
-5:14-5:30: @15.Call: _48 = Arguments::new_v1(move _49, move _53) -&gt; [return: bb16, unwind: bb29]
+5:14-5:30: @15.Call: _48 = std::fmt::Arguments::new_v1(move _49, move _53) -&gt; [return: bb16, unwind: bb29]
 5:14-5:30: @16.Call: core::panicking::panic_fmt(move _48) -&gt; bb29"><span class="annotation">@12,14,15,16⦊</span>assert_eq!(1, 1)<span class="annotation">⦉@12,14,15,16</span></span><span><span class="code even" style="--layer: 1" title="5:14-5:30: @13[0]: _0 = const ()"><span class="annotation">⦉@13</span></span></span><span class="code" style="--layer: 0">, // this,</span></span>
 <span class="line"><span class="code" style="--layer: 0">        3 =&gt; </span><span><span class="code odd" style="--layer: 1" title="6:14-6:30: @19[0]: _0 = const ()"><span class="annotation">@19⦊</span></span></span><span class="code even" style="--layer: 2" title="6:14-6:30: @18[5]: _144 = const fn_run_in_doctests::promoted[6]
 6:14-6:30: @18[6]: _85 = &amp;(*_144)
@@ -132,16 +132,16 @@
 6:14-6:30: @18[28]: _96 = (_91.0: &amp;&amp;i32)
 6:14-6:30: @18[30]: _97 = (_91.1: &amp;&amp;i32)
 6:14-6:30: @18[33]: _99 = &amp;(*_96)
-6:14-6:30: @18[35]: _100 = &lt;&amp;i32 as Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r &amp;i32, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
-6:14-6:30: @18.Call: _98 = ArgumentV1::new::&lt;&amp;i32&gt;(move _99, move _100) -&gt; [return: bb20, unwind: bb29]
+6:14-6:30: @18[35]: _100 = &lt;&amp;i32 as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r &amp;i32, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
+6:14-6:30: @18.Call: _98 = std::fmt::ArgumentV1::new::&lt;&amp;i32&gt;(move _99, move _100) -&gt; [return: bb20, unwind: bb29]
 6:14-6:30: @20[4]: _102 = &amp;(*_97)
-6:14-6:30: @20[6]: _103 = &lt;&amp;i32 as Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r &amp;i32, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
-6:14-6:30: @20.Call: _101 = ArgumentV1::new::&lt;&amp;i32&gt;(move _102, move _103) -&gt; [return: bb21, unwind: bb29]
+6:14-6:30: @20[6]: _103 = &lt;&amp;i32 as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r &amp;i32, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
+6:14-6:30: @20.Call: _101 = std::fmt::ArgumentV1::new::&lt;&amp;i32&gt;(move _102, move _103) -&gt; [return: bb21, unwind: bb29]
 6:14-6:30: @21[2]: _90 = [move _98, move _101]
 6:14-6:30: @21[7]: _89 = &amp;_90
 6:14-6:30: @21[8]: _88 = &amp;(*_89)
 6:14-6:30: @21[9]: _87 = move _88 as &amp;[std::fmt::ArgumentV1] (Pointer(Unsize))
-6:14-6:30: @21.Call: _82 = Arguments::new_v1(move _83, move _87) -&gt; [return: bb22, unwind: bb29]
+6:14-6:30: @21.Call: _82 = std::fmt::Arguments::new_v1(move _83, move _87) -&gt; [return: bb22, unwind: bb29]
 6:14-6:30: @22.Call: core::panicking::panic_fmt(move _82) -&gt; bb29"><span class="annotation">@18,20,21,22⦊</span>assert_eq!(1, 1)<span class="annotation">⦉@18,20,21,22</span></span><span><span class="code odd" style="--layer: 1" title="6:14-6:30: @19[0]: _0 = const ()"><span class="annotation">⦉@19</span></span></span><span class="code" style="--layer: 0">, // and this too</span></span>
 <span class="line"><span class="code" style="--layer: 0">        _ =&gt; </span><span><span class="code even" style="--layer: 1" title="7:14-7:30: @24[0]: _0 = const ()"><span class="annotation">@24⦊</span></span></span><span class="code even" style="--layer: 2" title="7:14-7:30: @23[5]: _147 = const fn_run_in_doctests::promoted[9]
 7:14-7:30: @23[6]: _119 = &amp;(*_147)
@@ -156,16 +156,16 @@
 7:14-7:30: @23[28]: _130 = (_125.0: &amp;&amp;i32)
 7:14-7:30: @23[30]: _131 = (_125.1: &amp;&amp;i32)
 7:14-7:30: @23[33]: _133 = &amp;(*_130)
-7:14-7:30: @23[35]: _134 = &lt;&amp;i32 as Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r &amp;i32, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
-7:14-7:30: @23.Call: _132 = ArgumentV1::new::&lt;&amp;i32&gt;(move _133, move _134) -&gt; [return: bb25, unwind: bb29]
+7:14-7:30: @23[35]: _134 = &lt;&amp;i32 as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r &amp;i32, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
+7:14-7:30: @23.Call: _132 = std::fmt::ArgumentV1::new::&lt;&amp;i32&gt;(move _133, move _134) -&gt; [return: bb25, unwind: bb29]
 7:14-7:30: @25[4]: _136 = &amp;(*_131)
-7:14-7:30: @25[6]: _137 = &lt;&amp;i32 as Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r &amp;i32, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
-7:14-7:30: @25.Call: _135 = ArgumentV1::new::&lt;&amp;i32&gt;(move _136, move _137) -&gt; [return: bb26, unwind: bb29]
+7:14-7:30: @25[6]: _137 = &lt;&amp;i32 as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r &amp;i32, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
+7:14-7:30: @25.Call: _135 = std::fmt::ArgumentV1::new::&lt;&amp;i32&gt;(move _136, move _137) -&gt; [return: bb26, unwind: bb29]
 7:14-7:30: @26[2]: _124 = [move _132, move _135]
 7:14-7:30: @26[7]: _123 = &amp;_124
 7:14-7:30: @26[8]: _122 = &amp;(*_123)
 7:14-7:30: @26[9]: _121 = move _122 as &amp;[std::fmt::ArgumentV1] (Pointer(Unsize))
-7:14-7:30: @26.Call: _116 = Arguments::new_v1(move _117, move _121) -&gt; [return: bb27, unwind: bb29]
+7:14-7:30: @26.Call: _116 = std::fmt::Arguments::new_v1(move _117, move _121) -&gt; [return: bb27, unwind: bb29]
 7:14-7:30: @27.Call: core::panicking::panic_fmt(move _116) -&gt; bb29"><span class="annotation">@23,25,26,27⦊</span>assert_eq!(1, 2)<span class="annotation">⦉@23,25,26,27</span></span><span><span class="code even" style="--layer: 1" title="7:14-7:30: @24[0]: _0 = const ()"><span class="annotation">⦉@24</span></span></span><span class="code" style="--layer: 0">, // however this is not</span></span>
 <span class="line"><span class="code" style="--layer: 0">    }</span></span>
 <span class="line"><span class="code" style="--layer: 0">}</span><span><span class="code odd" style="--layer: 1" title="9:2-9:2: @28.Return: return"><span class="annotation">@28⦊</span>‸<span class="annotation">⦉@28</span></span></span></span></div>
make[1]: Leaving directory '/d/a/rust/rust/src/test/run-make-fulldeps/coverage-spanview'
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
make[1]: *** [Makefile:50: doctest_crate] Error 1
------------------------------------------




failures:
    [run-make] run-make-fulldeps\coverage-spanview

test result: FAILED. 181 passed; 1 failed; 36 ignored; 0 measured; 0 filtered out; finished in 56.08s



command did not execute successfully: "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--rustdoc-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustdoc.exe" "--rust-demangler-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-tools-bin\\rust-demangler.exe" "--src-base" "D:\\a\\rust\\rust\\src/test\\run-make-fulldeps" "--build-base" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\run-make-fulldeps" "--stage-id" "stage2-x86_64-pc-windows-msvc" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "x86_64-pc-windows-msvc" "--host" "x86_64-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.1\\x64\\python3.exe" "--lldb-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.1\\x64\\python3.exe" "--gdb" "C:\\msys64\\usr\\bin\\gdb" "--llvm-version" "11.0.1-rust-1.51.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--llvm-bin-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin" "--cc" "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Enterprise\\VC\\Tools\\MSVC\\14.28.29333\\bin\\HostX64\\x64\\cl.exe" "--cxx" "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Enterprise\\VC\\Tools\\MSVC\\14.28.29333\\bin\\HostX64\\x64\\cl.exe" "--cflags" "-nologo -MT -Brepro" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap test --stage 2 --exclude src/test/ui --exclude src/tools/linkchecker
Build completed unsuccessfully in 0:58:13
Build completed unsuccessfully in 0:58:13
make: *** [Makefile:72: ci-subset-1] Error 1
