racket
#lang racket/base

(define-syntax-rule (define-hello)
  (define hello 0))

(define-hello)
(define hello 1)

(module+ inner
  (define use-hello hello))
