(defun nth (n list)
  (cond (eq n 0) (car list)
        true     (nth (- n 1) (cdr list))))
