
tmp.js:10
var itemTypes=["mod","externcrate","import","struct","enum","fn","type","static","trait","impl","tymethod","method","structfield","variant","macro","primitive","associatedtype","constant","associatedconstant","union","foreigntype","keyword"];exports.itemTypes = itemTypes;var MAX_LEV_DISTANCE=3;exports.MAX_LEV_DISTANCE = MAX_LEV_DISTANCE;var MAX_RESULTS=200;exports.MAX_RESULTS = MAX_RESULTS;var TY_PRIMITIVE=itemTypes.indexOf("primitive");exports.TY_PRIMITIVE = TY_PRIMITIVE;var levenshtein_row2=[];exports.levenshtein_row2 = levenshtein_row2;function buildHrefAndPath(item){var displayPath;var href;var type=itemTypes[item.ty];var name=item.name;if(type==='mod'){displayPath=item.path+'::';href=rootPath+item.path.replace(/::/g,'/')+'/'+name+'/index.html';}else if(type==="primitive"||type==="keyword"){displayPath="";href=rootPath+item.path.replace(/::/g,'/')+'/'+type+'.'+name+'.html';}else if(type==="externcrate"){displayPath="";href=rootPath+name+'/index.html';}else if(item.parent!==undefined){var mypar
ReferenceError: TY_KEYWORD is not defined
    at tmp.js:10:5209
    at Array.sort (native)
    at sortResults (tmp.js:10:4675)
    at execQuery (tmp.js:10:13651)
    at Object.execSearch (tmp.js:10:14529)
    at C:\projects\rust\src\tools\rustdoc-js\tester.js:260:30
    at Array.forEach (<anonymous>)
    at main (C:\projects\rust\src\tools\rustdoc-js\tester.js:253:33)
    at Object.<anonymous> (C:\projects\rust\src\tools\rustdoc-js\tester.js:301:14)
    at Module._compile (module.js:652:30)
command did not execute successfully: "C:\\Program Files (x86)\\nodejs\\node" "src/tools/rustdoc-js/tester.js" "x86_64-pc-windows-msvc"
expected success, got: exit code: 1
