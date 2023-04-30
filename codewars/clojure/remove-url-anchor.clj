(defn remove-url-anchor [url]
    (-> url
        (clojure.string/split #"#")
        first))

