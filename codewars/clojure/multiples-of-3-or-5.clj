; Codewars - Multiples of 3 or 5

(defn is-multiple [number]
    (or (= (mod number 3) 0)
        (= (mod number 5) 0)))

(defn calculate-sum [number result]
    (if (< number 1)
        result
        (recur (dec number)
               (if (is-multiple number)
                   (+ result number)
                   result))))

(defn solution [number]
    (calculate-sum (dec number) 0))

