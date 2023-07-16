sh
cd compiler/rustc_baked_icu_data
rm -r src/data
icu4x-datagen -W --pretty --fingerprint --use-separate-crates --format mod -l en es fr it ja pt ru tr zh zh-Hans zh-Hant -k list/and@1 fallback/likelysubtags@1 fallback/parents@1 fallback/supplement/co@1 --cldr-tag latest --icuexport-tag latest -o src/data
