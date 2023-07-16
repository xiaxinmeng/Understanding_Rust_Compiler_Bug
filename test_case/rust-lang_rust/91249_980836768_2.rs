racket
(module+ inner
  (require (only-in (submod "..") hello))
  (define use-hello hello))
