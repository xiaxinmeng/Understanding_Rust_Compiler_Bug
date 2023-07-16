
froydnj@hawkeye:/opt/build/froydnj/build-gcc$ readelf -sW ~/firefox/libxul.so|grep to_css | sort -k 3 -n -r |head -n 3 | awk '{print $3, $4, $5, $6, $8}'
86353 FUNC LOCAL DEFAULT _ZN86_$LT$style..properties..PropertyDeclaration$u20$as$u20$style_traits..values..ToCss$GT$6to_css17hd071bc295407688aE
55138 FUNC LOCAL DEFAULT _ZN86_$LT$style..properties..PropertyDeclaration$u20$as$u20$style_traits..values..ToCss$GT$6to_css17h2784d2352282c691E
34708 FUNC LOCAL DEFAULT _ZN86_$LT$style..properties..PropertyDeclaration$u20$as$u20$style_traits..values..ToCss$GT$6to_css17h82a7434b34298b9bE
