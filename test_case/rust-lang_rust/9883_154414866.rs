
\usepackage[T2A]{fontenc}
\usepackage[utf8]{inputenc}
\usepackage[english,russian]{babel}

...
% This insanity declares macros that emulate Russian letters, like \cyra

\DeclareSymbolFont{cyrillic}{T2A}{cmr}{m}{n}
\def\makecyrsymbol#1#2{%
    \begingroup\edef\temp{\endgroup
        \noexpand\DeclareMathSymbol{\noexpand#1}
        {\noexpand\mathalpha}{cyrillic}%
        {\expandafter\expandafter\expandafter
            \calccyr\expandafter\meaning\csname T2A\string#2\endcsname\end}}%
    \temp}
\expandafter\def\expandafter\calccyr\string\char#1\end{#1}

\makecyrsymbol\ca\cyra
\makecyrsymbol\be\cyrb
\makecyrsymbol\ve\cyrv
...

\makecyrsymbol\CA\CYRA
\makecyrsymbol\BE\CYRB
\makecyrsymbol\VE\CYRV
\makecyrsymbol\GE\CYRG
\makecyrsymbol\DE\CYRD
...
