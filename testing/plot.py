import matplotlib.pyplot as plt
import numpy as np
import sys

filename = sys.argv[1]

plt.xlabel('x')
plt.ylabel('y')

data = np.genfromtxt(filename, delimiter=' ')
plt.plot(data[:,0], data[:,1])

if filename == "testing/general":	
	x = [1, 2, 3, 4, 5, 5.5, 7, 8, 9, 9.5, 10]
	y = [0, 0, 0, 0.5, 0.4, 1.2, 1.2, 0.1, 0, 0.3, 0.6]
elif filename == "testing/step": 
	x = [1, 2, 3, 4, 5, 6, 7, 8]
	y = [-1, -1, -1, 0, 1, 1, 1, 1]
elif filename == "testing/line":
	x = [1, 2]
	y = [-1, -2]
elif filename == "testing/basic":
	x = [1, 2, 3]
	y = [3, 5, 2]
plt.plot(x,y, 'ro')

plt.show()