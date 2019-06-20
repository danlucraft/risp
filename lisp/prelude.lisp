(defun assert_eq! (x y)
  (cond (eq x y) true
        true     (do (prn x y) (assert! false))))

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
  (cond (eq y '())           nil
        (eq x (car (car y))) (car (cdr (car y)))
        true                 (lookup x (cdr y))))

(assert_eq! 'a (lookup 1 '((1 a))))
(assert_eq! 'b (lookup 101 '((101 b) (1 a))))
(assert_eq! 'a (lookup 1 '((101 b) (1 a))))
(assert_eq! nil (lookup 2 '((101 b) (1 a))))

(defun eval (e a)
  (cond
    (atom e)  (lookup e a)
    (int? e)  e
    (bool? e) e
    (nil? e)  e
    (atom (car e))
      (cond
        (eq (car e) 'quote) (car (cdr e))
        (eq (car e) 'atom)  (atom  (eval (car (cdr e)) a))
        (eq (car e) 'eq)    (eq    (eval (car (cdr e)) a)
                                   (eval (nth 2 e) a))
        (eq (car e) 'car)   (car   (eval (car (cdr e)) a))
        (eq (car e) 'cdr)   (cdr   (eval (car (cdr e)) a))
        (eq (car e) 'cons)  (cons  (eval (car (cdr e)) a)
                                   (eval (nth 2 e) a))
        (eq (car e) 'cond)  (evcon (cdr e) a)
        (eq (car e) '+)     (+ (eval (nth 1 e) a) 
                               (eval (nth 2 e) a))
        true (eval (cons (lookup (car e) a)
                         (cdr e))
                   a))
    (eq (car (car e)) 'label)
      (eval (cons (nth 2 (car e)) (cdr e))
            (cons (list (nth 1 (car e)) (car e)) a))
    (eq (car (car e)) 'lambda)
      (eval (nth 2 (car e)) 
            (append (zip (nth 1 (car e)) (evlis (cdr e) a))
                    a))))

(defun evcon (c a)
  (cond (eval (car c) a) (eval (nth 1 c) a)
        true             (evcon (cdr (cdr c)) a)))

(defun evlis (m a)
  (cond (null? m) '()
        true      (cons (eval (car m) a)
                        (evlis (cdr m) a))))

(assert_eq! 3 (eval '(+ 1 2) '()))
(assert_eq! '(1 1 2) (eval '(cons 1 '(1 2)) '()))
(assert_eq! '(1 10 2) (eval '(cons 1 (cons a '(2))) '((a 10))))
(assert_eq! 1 (eval '(car '(1 2)) '()))
(assert_eq! '(2) (eval '(cdr '(1 2)) '()))
(assert_eq! false (eval '(eq 1 2) '()))

(assert_eq! 'a (eval 'x '((x a) (y b))))
(assert_eq! '(a b c) (eval '(cons x '(b c)) '((x a) (y b))))

(assert_eq! 'list (eval '(cond (atom x) 'atom true 'list) '((x '(a b)))))

(assert_eq! '(a b c) (eval '(f '(b c)) '((f (lambda (x) (cons 'a x))))))

(assert_eq! 
  'a 
  (eval '(
    (label firstatom 
           (lambda (x)
             (cond (atom x) x
                   true     (firstatom (car x)))))
    y)
   '((y ((a b) (c d))))))

(assert_eq!
  '(a c d)
  (eval
    '((lambda (x y) (cons x (cdr y)))
      'a
      '(b c d))
    '()))
