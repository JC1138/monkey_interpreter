let tricky = function(a, b) {
    return a + b * function(c) {
        if (c < 10) {
            return c + function(d) {
                return d * function(e) {
                    return e - function(f) {
                        return Math.floor(f / 2);
                    }(d);
                }(c);
            }(b);
        } else {
            return c - function(g) {
                return g + a;
            }(b);
        }
    }(a);
};

console.log(tricky(1, 3))