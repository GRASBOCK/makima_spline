import matplotlib.pyplot as plt
import numpy as np

plt.xlabel('x')
plt.ylabel('y')

data = np.genfromtxt("general", delimiter=' ')
plt.plot(data[:,0], data[:,1])

x = [1, 2, 3, 4, 5, 5.5, 7, 8, 9, 9.5, 10]
y = [0, 0, 0, 0.5, 0.4, 1.2, 1.2, 0.1, 0, 0.3, 0.6]
plt.plot(x,y, 'ro')

plt.show()