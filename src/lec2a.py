# rust 限制太多了2333
def sqrt(x):
    def close_enough(a, b):
        return abs(a-b) < 0.00001

    def f(start):
        return (start+x/start)/2

    def iter(old, new):
        if close_enough(old, new):
            return new
        else:
            return iter(new, f(new))

    def find_fix_point(f, start):
        return iter(f(start), start)

    return find_fix_point(f, 1)


def sqrt2(x):
    a = 1
    v = (a+x/a)/2
    while abs(v-a) > 0.0001:
        a = v
        v = (a+x/a)/2
    return v

print(sqrt2(2))