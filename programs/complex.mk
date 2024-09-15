let complex = fn(x) {
    return fn(y) {
        let z = fn(w) {
            return x + y + w;
        }(fn(v) {
            return v * x;
        }(y));
        return z / (x - y);
    }(x * 2);
};
