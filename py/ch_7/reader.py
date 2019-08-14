'''
Created on Jul 14, 2019

@author: notav

fmap :: (a -> b) -> (r -> a) -> (r -> b)

'''

'''
get these to recurse
'''

class maybe():
    def __init__(self,value=None,Error=None):
        
        self.value = value
        self.Error=Error
    
    def get(self):
        if self.Error:
            return self.Error
        else:
            return self.value
    
    def fmap(self,f):
        if self.Error:
            return maybe(Error=self.Error)
        else:
            return maybe(f(self.value))

def iden(x):
    return x

class reader():
    def __init__(self,r):
        self.r = r
    
    def fmap(self,f, b):
        def retfunc():
            return b(f(self.r()))
        return retfunc

        

if __name__ == "__main__":
    ntup = (1, (2,3))
    
    from random import randint
    
    
    randReader = reader(lambda : randint(0,5))
    
    b = int
    
    f = lambda x : x*10 + 5
    
    for _ in range(10):
        print (randReader.fmap(f,b)())
        
    f2 = lambda x : 'value is : ' + str(x)
    
    for _ in range(10):
        print(*randReader.fmap(f2,str)())