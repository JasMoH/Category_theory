'''
Created on Jul 15, 2019

@author: notav
'''
    

def bimap(f,g):
    def fab(a,b):
        return (f(a),g(b))
    return fab

if __name__ == '__main__':
    fab = bimap(lambda x : x + 1, lambda x : ''.join([str.capitalize(a) for a in x] ))
    
    print(fab(1,'hello'))