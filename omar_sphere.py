import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
import matplotlib
matplotlib.use('Agg') 

fig = plt.figure(figsize=(10, 10))
ax = fig.add_subplot(111, projection='3d')

z = np.linspace(0, 15, 100)
ax.plot(np.zeros(100)+0.5, np.zeros(100), z, color='gold', linewidth=5, label='Critical Line 0.5')

t = np.linspace(0, 8*np.pi, 500)
z_scroll = np.linspace(0, 15, 500)
r = 1.0 
x = 0.5 + r * np.cos(t)
y = r * np.sin(t)
ax.plot(x, y, z_scroll, color='cyan', alpha=0.5, label='Folding Scroll (10^177)')

z_zeros = np.arange(2, 14, 2)
ax.scatter(np.zeros(len(z_zeros))+0.5, np.zeros(len(z_zeros)), z_zeros, color='red', s=200, label='Zeros Locked at 0.5')

ax.set_title("Omar Al-Mutairi: Riemann Sphere & Scroll Theory\nKing Faisal University", fontsize=12)
ax.legend()

plt.savefig('omar_proof_vision.png', dpi=300)
print("✅ Done! File saved as omar_proof_vision.png")
