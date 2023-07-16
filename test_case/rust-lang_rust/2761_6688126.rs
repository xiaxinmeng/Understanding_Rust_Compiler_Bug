
#macro[
    [#assert[cond], if !(cond) { fail }],
    [#assert[cond, message], if !(cond) { fail message }]
];
