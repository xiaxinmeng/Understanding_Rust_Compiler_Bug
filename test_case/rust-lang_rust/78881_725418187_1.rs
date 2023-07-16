llvm
; ModuleID = 'bugpoint-reduced-simplified.bc'
source_filename = "main.7rcbfp3g-cgu.6"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "aarch64-apple-macosx10.7.0"

%0 = type { [0 x i8], {}, [0 x i8] }
%1 = type { [0 x i64], { i64, i64 }, [0 x i64], { i64, i64 }, [0 x i64], { {}*, [3 x i64]* }, [0 x i32], i32, [0 x i32], i32, [0 x i8], i8, [7 x i8] }
%2 = type { [0 x i64], i64, [0 x i64], void (i32, %2*)*, [0 x i64], [2 x i64], [0 x i64] }
%3 = type { [0 x i8] }
%4 = type { [0 x i64], { [0 x { [0 x i8]*, i64 }]*, i64 }, [0 x i64], { i64*, i64 }, [0 x i64], { [0 x { i8*, i64* }]*, i64 }, [0 x i64] }

@anon.46a3ec0d5d41ada7f589fff33b70a177.0 = external dso_local unnamed_addr constant <{ [16 x i8] }>, align 1
@anon.46a3ec0d5d41ada7f589fff33b70a177.1 = external dso_local unnamed_addr constant { void (%0*)*, i64, i64, i1 (%0*, %1*)* }, align 8
@anon.46a3ec0d5d41ada7f589fff33b70a177.8 = external dso_local unnamed_addr constant <{ i8*, [16 x i8] }>, align 8
@anon.46a3ec0d5d41ada7f589fff33b70a177.6 = external dso_local unnamed_addr constant <{ i8*, [16 x i8] }>, align 8
@anon.257d62b6b55dab08f3f342747a99f557.10.llvm.6722910066062874695 = external dso_local unnamed_addr constant <{ [36 x i8] }>, align 1
@anon.257d62b6b55dab08f3f342747a99f557.11.llvm.6722910066062874695 = external dso_local unnamed_addr constant <{ i8*, [16 x i8] }>, align 8

declare dso_local void @_ZN4core6result13unwrap_failed17h2509b0063399a516E() unnamed_addr #0

declare dso_local void @_ZN4core9panicking5panic17h71deb15c6ab1b2f2E() unnamed_addr #0

declare i32 @rust_eh_personality(i32, i32, i64, %2*, %3*) unnamed_addr #0

declare dso_local i1 @_ZN4core10intrinsics17is_nonoverlapping17h00310cca42bb40ecE({ [0 x i8]*, i64 }*, { [0 x i8]*, i64 }*, i64) unnamed_addr #0

declare dso_local void @_ZN3std9panicking15begin_panic_fmt17h2baa5a44d3c7d0dcE() unnamed_addr #0

declare dso_local void @"_ZN57_$LT$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$9write_fmt17h742ae361023605ddE"() unnamed_addr #0

declare dso_local void @"_ZN4core4cell13Cell$LT$T$GT$7replace17h1b2ba7594bdf55acE"() unnamed_addr #0

declare dso_local fastcc void @_ZN4core3ptr13drop_in_place17hf676aeaf907fa742E() unnamed_addr #0

define dso_local void @_ZN3std2io5stdio8print_to17h83af8c48359573cfE(i64* ()* %0) unnamed_addr #0 personality i32 (i32, i32, i64, %2*, %3*)* @rust_eh_personality {
  br i1 undef, label %64, label %2

2:                                                ; preds = %1
  br i1 undef, label %64, label %3

3:                                                ; preds = %2
  br i1 undef, label %5, label %4

4:                                                ; preds = %3
  unreachable

5:                                                ; preds = %3
  %6 = invoke zeroext i1 bitcast (i1 ({ [0 x i8]*, i64 }*, { [0 x i8]*, i64 }*, i64)* @_ZN4core10intrinsics17is_nonoverlapping17h00310cca42bb40ecE to i1 ({ i8*, i8* }*, { i8*, i8* }*, i64)*)({ i8*, i8* }* nonnull undef, { i8*, i8* }* nonnull undef, i64 1)
          to label %7 unwind label %15

7:                                                ; preds = %5
  br i1 %6, label %9, label %8

8:                                                ; preds = %7
  unreachable

9:                                                ; preds = %7
  %10 = invoke zeroext i1 bitcast (i1 ({ [0 x i8]*, i64 }*, { [0 x i8]*, i64 }*, i64)* @_ZN4core10intrinsics17is_nonoverlapping17h00310cca42bb40ecE to i1 ({ i8*, i8* }*, { i8*, i8* }*, i64)*)({ i8*, i8* }* nonnull undef, { i8*, i8* }* nonnull undef, i64 1)
          to label %11 unwind label %13

11:                                               ; preds = %9
  br i1 %10, label %18, label %12

12:                                               ; preds = %11
  unreachable

13:                                               ; preds = %9
  %14 = landingpad { i8*, i32 }
          cleanup
  invoke fastcc void @_ZN4core3ptr13drop_in_place17hf676aeaf907fa742E() #1
          to label %17 unwind label %15

15:                                               ; preds = %13, %5
  %16 = landingpad { i8*, i32 }
          cleanup
  br label %17

17:                                               ; preds = %13, %15
  invoke fastcc void @_ZN4core3ptr13drop_in_place17hf676aeaf907fa742E() #1
          to label %53 unwind label %51

18:                                               ; preds = %11
  br i1 undef, label %21, label %19

19:                                               ; preds = %18
  invoke void @_ZN4core9panicking5panic17h71deb15c6ab1b2f2E()
          to label %20 unwind label %49

20:                                               ; preds = %19
  unreachable

21:                                               ; preds = %18
  invoke void @"_ZN4core4cell13Cell$LT$T$GT$7replace17h1b2ba7594bdf55acE"()
          to label %22 unwind label %49

22:                                               ; preds = %21
  br i1 undef, label %23, label %25

23:                                               ; preds = %22
  %24 = invoke align 8 dereferenceable(104) i64* %0()
          to label %34 unwind label %49

25:                                               ; preds = %22
  %26 = invoke i128 undef({}* nonnull align 1 undef, %4* noalias nocapture nonnull dereferenceable(48) undef)
          to label %27 unwind label %45

27:                                               ; preds = %25
  br i1 undef, label %30, label %28

28:                                               ; preds = %27
  invoke void @_ZN4core6result13unwrap_failed17h2509b0063399a516E()
          to label %29 unwind label %58

29:                                               ; preds = %28
  unreachable

30:                                               ; preds = %27
  invoke void @"_ZN4core4cell13Cell$LT$T$GT$7replace17h1b2ba7594bdf55acE"()
          to label %31 unwind label %58

31:                                               ; preds = %30
  br i1 undef, label %41, label %32

32:                                               ; preds = %31
  invoke void @_ZN4core3ptr13drop_in_place17h4425f9802295111aE()
          to label %41 unwind label %38

33:                                               ; preds = %44
  br label %62

34:                                               ; preds = %23
  invoke void @"_ZN57_$LT$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$9write_fmt17h742ae361023605ddE"()
          to label %35 unwind label %54

35:                                               ; preds = %34
  br i1 undef, label %37, label %36

36:                                               ; preds = %35
  invoke void @_ZN4core3ptr13drop_in_place17h4425f9802295111aE()
          to label %37 unwind label %56

37:                                               ; preds = %36, %35
  br label %62

38:                                               ; preds = %32
  %39 = landingpad { i8*, i32 }
          cleanup
  br label %47

40:                                               ; preds = %56, %54, %53, %49, %48, %47, %45
  resume { i8*, i32 } undef

41:                                               ; preds = %32, %31
  br i1 undef, label %44, label %42

42:                                               ; preds = %41
  invoke void @_ZN4core9panicking5panic17h71deb15c6ab1b2f2E()
          to label %43 unwind label %60

43:                                               ; preds = %42
  unreachable

44:                                               ; preds = %41
  invoke void @"_ZN4core4cell13Cell$LT$T$GT$7replace17h1b2ba7594bdf55acE"()
          to label %33 unwind label %60

45:                                               ; preds = %25
  %46 = landingpad { i8*, i32 }
          cleanup
  br label %40

47:                                               ; preds = %60, %58, %38
  br label %40

48:                                               ; preds = %49
  br label %40

49:                                               ; preds = %21, %19, %23
  %50 = landingpad { i8*, i32 }
          cleanup
  br i1 undef, label %40, label %48

51:                                               ; preds = %17
  %52 = landingpad { i8*, i32 }
          cleanup
  br label %53

53:                                               ; preds = %17, %51
  br label %40

54:                                               ; preds = %34
  %55 = landingpad { i8*, i32 }
          cleanup
  br label %40

56:                                               ; preds = %36
  %57 = landingpad { i8*, i32 }
          cleanup
  br label %40

58:                                               ; preds = %30, %28
  %59 = landingpad { i8*, i32 }
          cleanup
  br label %47

60:                                               ; preds = %44, %42
  %61 = landingpad { i8*, i32 }
          cleanup
  br label %47

62:                                               ; preds = %37, %33
  %63 = phi i120 [ undef, %37 ], [ undef, %33 ]
  br i1 undef, label %64, label %65

64:                                               ; preds = %62, %2, %1
  br label %70

65:                                               ; preds = %62
  %66 = zext i120 %63 to i128
  %67 = shl nuw i128 %66, 8
  %68 = select i1 undef, i128 0, i128 %67
  %69 = or i128 %68, 0
  br label %70

70:                                               ; preds = %65, %64
  %71 = phi i128 [ undef, %64 ], [ %69, %65 ]
  %72 = trunc i128 %71 to i8
  %73 = icmp eq i8 %72, 3
  br i1 %73, label %74, label %75

74:                                               ; preds = %70
  ret void

75:                                               ; preds = %70
  invoke void @_ZN3std9panicking15begin_panic_fmt17h2baa5a44d3c7d0dcE()
          to label %78 unwind label %76

76:                                               ; preds = %75
  %77 = landingpad { i8*, i32 }
          cleanup
  resume { i8*, i32 } %77

78:                                               ; preds = %75
  unreachable
}

declare dso_local void @_ZN4core3ptr13drop_in_place17h4425f9802295111aE() unnamed_addr #0

attributes #0 = { "target-cpu"="apple-a12" }
attributes #1 = { noinline }

!llvm.module.flags = !{!0}

!0 = !{i32 2, !"Debug Info Version", i32 3}
