; Codewars - Playing with digits

(defn find-factor [number result value]
    (if (= (* number value) result)
        value
        (if (> (* number value) result)
            -1
            (recur number result (inc value)))))

(defn find-result [numbers exponent result]
    (if (empty? (rest numbers))
        (+ result (Math/pow (first numbers) exponent))
        (recur
            (rest numbers)
            (inc exponent)
            (+ result (Math/pow (first numbers) exponent)))))

(defn dig-pow [n p]
    (find-factor n 
        (bigint (find-result 
            (map #(Integer/parseInt %) (clojure.string/split (str n) #""))
            p 0)) 1))

