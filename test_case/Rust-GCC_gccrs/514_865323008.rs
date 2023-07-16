lisp
(defun my-c++-mode-hook ()
  (require 'clang-format)
  (defun clang-format-buffer-smart ()
    "Reformat buffer if .clang-format exists in the projectile root."
    (when (eq major-mode 'c++-mode)
      (clang-format-buffer "file" (expand-file-name "contrib/clang-format" (projectile-project-root)))))
  
  (add-hook 'before-save-hook 'clang-format-buffer-smart)
  
  (require 'lsp-mode)
  (require 'lsp-ui)
  (require 'company)

  (setq lsp-print-io t)
  (setq lsp-enable-snippet t)
  (setq lsp-signature-render-documentation t)

  (local-set-key (kbd "C-c C-j") 'lsp-find-definition)
  (global-set-key (kbd "C-<tab>") 'company-complete)

  (lsp 1)
  (lsp-ui-mode 1)
  )
