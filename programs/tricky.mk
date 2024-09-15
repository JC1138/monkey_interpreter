let tricky = fn(a, b) {
    return a + b * fn(c) {
        if (c < 10) {
            return c + fn(d) {
                return d * fn(e) {
                    return e - fn(f) {
                        return f / 2;
                    }(d);
                }(c);
            }(b);
        } else {
            return c - fn(g) {
                return g + a;
            }(b);
        }
    }(a);
};
