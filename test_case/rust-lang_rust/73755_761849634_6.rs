
wasm-ld-11: Loading: test.o
wasm-ld-11: warning: creating shared libraries, with -shared, is not yet stable
wasm-ld-11: Processing: test.o
wasm-ld-11: -- createOutputSegments
wasm-ld-11: -- createSyntheticSections
wasm-ld-11: -- populateProducers
wasm-ld-11: -- populateTargetFeatures
wasm-ld-11: -- calculateImports
wasm-ld-11: -- layoutMemory
wasm-ld-11: mem: global base = 0
wasm-ld-11: mem: static data = 0
wasm-ld-11: -- scanRelocations
wasm-ld-11: -- assignIndexes
wasm-ld-11: -- calculateInitFunctions
wasm-ld-11: -- calculateTypes
wasm-ld-11: -- calculateExports
wasm-ld-11: -- calculateCustomSections
wasm-ld-11: calculateCustomSections
wasm-ld-11: -- populateSymtab
wasm-ld-11: -- addSections
wasm-ld-11: addSection: CUSTOM(dylink)
wasm-ld-11: addSection: TYPE
wasm-ld-11: addSection: IMPORT
wasm-ld-11: addSection: FUNCTION
wasm-ld-11: addSection: EXPORT
wasm-ld-11: addSection: CODE
wasm-ld-11: createCustomSections
wasm-ld-11: addSection: CUSTOM(name)
wasm-ld-11: addSection: CUSTOM(target_features)
wasm-ld-11: Defined Functions: 3
wasm-ld-11: Defined Globals  : 0
wasm-ld-11: Defined Events   : 0
wasm-ld-11: Function Imports : 0
wasm-ld-11: Global Imports   : 2
wasm-ld-11: Event Imports    : 0
wasm-ld-11: info for: test.o
              Symbols : 1
     Function Imports : 0
       Global Imports : 0
        Event Imports : 0
wasm-ld-11: -- finalizeSections
wasm-ld-11: setOffset: CUSTOM(dylink): 8
wasm-ld-11: createHeader: CUSTOM(dylink) body=12 total=14
wasm-ld-11: setOffset: TYPE: 22
wasm-ld-11: createHeader: TYPE body=10 total=12
wasm-ld-11: setOffset: IMPORT: 34
wasm-ld-11: createHeader: IMPORT body=90 total=92
wasm-ld-11: setOffset: FUNCTION: 126
wasm-ld-11: createHeader: FUNCTION body=4 total=6
wasm-ld-11: setOffset: EXPORT: 132
wasm-ld-11: createHeader: EXPORT body=29 total=31
wasm-ld-11: setOffset: CODE: 163
wasm-ld-11: createHeader: CODE body=22 total=24
wasm-ld-11: setOffset: CUSTOM(name): 187
wasm-ld-11: createHeader: CUSTOM(name) body=55 total=57
wasm-ld-11: setOffset: CUSTOM(target_features): 244
wasm-ld-11: createHeader: CUSTOM(target_features) body=29 total=31
wasm-ld-11: -- openFile
wasm-ld-11: writing: test.wasm
wasm-ld-11: -- writeSections
wasm-ld-11: writing CUSTOM(target_features)
wasm-ld-11: writing CUSTOM(name)
wasm-ld-11: writing CODE
wasm-ld-11:  size=24
wasm-ld-11:  headersize=2
wasm-ld-11: writing FUNCTION
wasm-ld-11: writing IMPORT
wasm-ld-11:  codeheadersize=1
wasm-ld-11: writing EXPORT
wasm-ld-11: writing TYPE
wasm-ld-11: writing CUSTOM(dylink
