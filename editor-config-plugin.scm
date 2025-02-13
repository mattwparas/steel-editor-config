(require "editor-config.scm")
(require "helix/configuration.scm")
(require (prefix-in helix. "helix/commands.scm"))
(require (prefix-in helix. "helix/editor.scm"))

;; Config at path
(define (configure-opened-doc path)
  (with-handler (lambda (e)
                  ;; Leaves much to be desired, but this
                  ;; just logs the error
                  (log::info! (to-string e))
                  #f)
                ;; Set up to apply the rules to this doc when it gets opened
                (begin
                  (define config (config-at-path path))
                  (define line-endings (get-property config "end_of_line"))
                  (define indent-style-config (get-property config "indent_style"))
                  (define indent-size-length (get-property config "indent_size"))
                  (when line-endings
                    (helix.line-ending line-endings))
                  (cond
                    [(equal? indent-style-config "tab") (helix.indent-style "t")]
                    [(equal? indent-style-config "space")
                     (when indent-size-length
                       (helix.indent-style indent-size-length))]))))

(define (enable-editor-config)
  (define config (config-at-path "*"))
  (define insert-final-newline? (get-property config "insert_final_newline"))
  (when insert-final-newline?
    (insert-final-newline #t))
  (register-hook! 'document-opened
                  (lambda (id) (configure-opened-doc (~> id helix.editor-document->path)))))
