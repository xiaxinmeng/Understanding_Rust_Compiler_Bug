
git bisect start
# bad: [c284f8807eb3a1d728242bb6a767b0306d6f6bd5] Auto merge of #46789 - Diggsey:command-env-capture, r=dtolnay
git bisect bad c284f8807eb3a1d728242bb6a767b0306d6f6bd5
# good: [16992930835ce3376a4aaed42307726e1fc78e45] Auto merge of #46864 - estebank:closure-type-err-sp, r=nikomatsakis
git bisect good 16992930835ce3376a4aaed42307726e1fc78e45
# good: [16992930835ce3376a4aaed42307726e1fc78e45] Auto merge of #46864 - estebank:closure-type-err-sp, r=nikomatsakis
git bisect good 16992930835ce3376a4aaed42307726e1fc78e45
# good: [11a24d9c3940f60e527c571680d64e80e0889abe] Auto merge of #46888 - cramertj:nested-impl-trait-error, r=nikomatsakis
git bisect good 11a24d9c3940f60e527c571680d64e80e0889abe
# good: [304717bd86e42bc9b0c45ea5a6068e7ed9d13f2f] Auto merge of #46894 - detrumi:fix-const-eval-trait, r=eddyb
git bisect good 304717bd86e42bc9b0c45ea5a6068e7ed9d13f2f
