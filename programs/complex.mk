let complex = fn(x) {
    return fn(y) {
        let z = fn(w) {
            println(x + w);
            return x + y + w;
        }(fn(v) {
            return v * x;
        }(y));
        return z / (x - y);
    }(x * 2);
};

complex(5)
