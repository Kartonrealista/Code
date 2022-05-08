class Forest:
    def __init__(self, x):
        self.parent = x
        self.x = x
    def MakeSet(self, x):
        if not isinstance(x, Forest.x):
            x = Forest(x)
            
    def Find(self, x):
        while x.parent != x:
            x = x.parent
            x.parent = x.parent.parent
        return x
    def Union(self, x, y):
        x = self.Find(x)
        y = self.Find(y)
        
        if x == y:
            return
        
        y.parent = x
