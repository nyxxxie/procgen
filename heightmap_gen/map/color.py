class Color(object):
    def __init__(self, r, g=-1, b=-1):
        if g == -1 and b == -1:
            self.set(r, r, r)
        else:
            self.set(r, g, b)

    def set(self, r, g, b):
        self.r = r
        self.g = g
        self.b = b


