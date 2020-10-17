import matplotlib.pyplot as plt
import numpy as np

plt.xlabel('x')
plt.ylabel('y')

data = np.genfromtxt("step", delimiter=' ')
plt.plot(data[:,0], data[:,1])

x = [1, 2, 3, 4, 5, 6, 7, 8]
y = [-1, -1, -1, 0, 1, 1, 1, 1]
plt.plot(x,y, 'ro')

plt.show()