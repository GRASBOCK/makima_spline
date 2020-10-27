import matplotlib.pyplot as plt
from matplotlib import cm
from matplotlib.ticker import LinearLocator, FormatStrFormatter
import numpy as np
import sys

# Make data.
filename = sys.argv[1]

data = np.genfromtxt(filename, delimiter=' ')
X = data[:,0]
Y = data[:,1]
Z = data[:,2]

x_steps = 60
y_steps = 90
X = np.reshape(X, (y_steps, x_steps))
Y = np.reshape(Y, (y_steps, x_steps))
Z = np.reshape(Z, (y_steps, x_steps))
print(X)
print(Y)
#X = np.array([[-2.5, 0.0, 1.5],[-2.5, 0.0, 1.5]])
#Y = np.array([[-4.5, -4.5, -4.5],[3.2, 3.2, 3.2]])
#Z = np.array([[12.4, 1.45, 1.33],[13.4, 13.2, 6.]])
#Z.split(data.shape())

# Plot the surface.

fig = plt.figure()
ax = fig.add_subplot(111, projection='3d')

ax.plot_surface(X, Y, Z, cmap=cm.coolwarm, antialiased=False)

ax.set_xlabel('X Label')
ax.set_ylabel('Y Label')
ax.set_zlabel('Z Label')

plt.show()