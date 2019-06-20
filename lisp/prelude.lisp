(defun nth (n list)
  (cond (eq n 0) (car list)
        true     (nth (- n 1) (cdr list))))

(defun null? (x)
  (eq x '()))

(defun and (x y)
  (cond x    (cond y    true
                   true false)
        true false))

(defun not (x)
  (cond x    false
        true true))

(defun append (x y)
  (cond (null? x) y
        true      (cons (car x) (append (cdr x) y))))

(defun zip (x y)
  (cons ))

