xctrace record --template 'Allocations' --launch ./target/release/oxy_lisp "(defn sum-to [n] (if (= n 0) n (+ n (sum-to (+ n -1))))) (sum-to 6 (fn [x] x))"
xctrace record --template 'Allocations' --launch ./target/release/oxy_lisp "(defn fib [n] (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2))))) (fib 20)"

