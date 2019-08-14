'''
Created on Jul 13, 2019

@author: notav
'''

class Either():
    def __init__(self,a=None,b=None):
        self.a = a
        self.b = b
        
        if (a != None) and (b !+ None):
            raise TypeError('cannot define a and b simultaneosly')
    

def factorizer(i,j,e):
    if e.a:
        return i(e.a)
    else:
        return j(e.b)

if __name__ == "__main__":
    i = lambda x : x
    j = lambda x : 1 if x else 0
    
    e1 = Either(a=15)
    e2 = Either(b=False)
    
    print('i(e1) = ',factorizer(i,j,e1))
    print('i(e2) = ',factorizer(i,j,e2))