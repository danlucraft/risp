(defun assert_eq! (x y)
  (assert! (eq x y)))

(defun nth (n list)
  (cond (eq n 0) (car list)
        true     (nth (- n 1) (cdr list))))

(assert_eq! 1 (nth 0 '(1 2 3)))
(assert_eq! 2 (nth 1 '(1 2 3)))
(assert_eq! 3 (nth 2 '(1 2 3)))

(defun null? (x)
  (eq x '()))

(assert_eq! true  (null? '()))
(assert_eq! false (null? 123))

(defun and (x y)
  (cond x    (cond y    true
                   true false)
        true false))

(assert_eq! true  (and true true))
(assert_eq! false (and true false))
(assert_eq! true  (and (eq 1 1) (eq true true)))
(assert_eq! false (and (eq 1 2) (eq true true)))

(defun or (x y)
  (cond x    true
        true (cond y    true
                   true false)))

(assert_eq! true  (or true true))
(assert_eq! true  (or true false))
(assert_eq! false (or false false))
(assert_eq! true  (or (eq 1 1) (eq true true)))
(assert_eq! true  (or (eq 1 2) (eq true true)))
(assert_eq! false (or (eq 1 2) (eq true false)))

(defun not (x)
  (cond x    false
        true true))

(assert_eq! false (not true))
(assert_eq! true  (not false))

(defun append (x y)
  (cond (null? x) y
        true      (cons (car x) (append (cdr x) y))))

(assert_eq! '(1 2 3 4) (append '(1 2) '(3 4)))
(assert_eq! '(3 4)     (append '() '(3 4)))
(assert_eq! '(1 2)     (append '(1 2) '()))

(defun list (x y)
  (cons x (cons y '())))

(assert_eq! '(1 2) (list 1 2))

(defun zip (x y)
  (cond (or (null? x) (null? y)) '()
        true                     (cons (list (car x) (car y)) (zip (cdr x) (cdr y)))))

(assert_eq! '((1 3) (2 4)) (zip '(1 2) '(3 4)))
(assert_eq! '((1 3)) (zip '(1 2) '(3)))
(assert_eq! '((2 4)) (zip '(2) '(4)))
(assert_eq! '() (zip '() '(4)))

(defun lookup (x y)
  (cond (eq x (car (car y))) (car (cdr (car y)))
        true                 (lookup x (cdr y))))

(assert_eq! 'a (lookup 1 '((1 a))))
(assert_eq! 'b (lookup 101 '((101 b) (1 a))))
(assert_eq! 'a (lookup 1 '((101 b) (1 a))))
