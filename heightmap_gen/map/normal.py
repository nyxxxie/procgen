import math
import numpy as np
from .base import Map
from .color import Color

def normalize(v):
    norm=np.linalg.norm(v, ord=1)
    if norm==0:
        norm=np.finfo(v.dtype).eps
    return v/norm

class NormalMap(Map):
    def __init__(self, width, height):
        super().__init__(width, height)

    @classmethod
    def from_heightmap(cls, heightmap):
        c = cls(heightmap.width, heightmap.height)

        def calc_normal(x, y):
            """Calculates normal vector for a coordinate x,y on the heightmap.

            Math used is based on the following article:
            https://squircleart.github.io/shading/normal-map-generation.html
            """
            # Get the 3d coordinates of each point adjacent to x,y
            u = float(heightmap.get_col((y+1) % c.height, x).r)
            d = float(heightmap.get_col((y-1) % c.height, x).r)
            r = float(heightmap.get_col(y, (x+1) % c.width).r)
            l = float(heightmap.get_col(y, (x-1) % c.width).r)

            # Calc linear derivatives
            dzdx = (l - r)/2
            dzdy = (d - u)/2

            # Calculate normal
            norm = normalize(np.array([-dzdx, -dzdy, 1.0]))

            # Convert normals to colors so we can encode them as pixels
            norm = (norm+1) * 128

            return Color(norm[0], norm[1], norm[2])

        c.map_coords(calc_normal)

        return c
