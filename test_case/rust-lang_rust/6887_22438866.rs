
(re-search-forward
 (cadr (assq 'gnu compilation-error-regexp-alist-alist)))   ;; CASE 1

nonexhaust.rs:2:4: 4:5 error: non-exhaustive patterns: None not covered

(re-search-forward
 (cadr (assq 'gnu compilation-error-regexp-alist-alist)))   ;; CASE 2

nonexhaust.rs:2:10: 2:15 error: expected `;` or `}` after expression but found `stuff`
