#!/usr/bin/env python
import math
from PIL import Image
from opensimplex import OpenSimplex
from progress.bar import Bar
import numpy as np

class Color(object):
    def __init__(self, r, g, b):
        self.r = r
        self.g = g
        self.b = b

class MapGenerator(object):
    def __init__(self, height, width, feature_size=24.0):
        self.width = width
        self.height = height
        self.feature_size = feature_size

        self.heightmap = np.zeros((height, width, 3), dtype=np.uint8)
        self.normalmap = self.heightmap.copy()

    def generate(self, progress=True):
        self._gen_heightmap(progress)
        self._gen_normalmap(progress)

    def export_heightmap(self, path):
        self._numpy_to_image(self.heightmap, path)

    def export_normalmap(self, path):
        self._numpy_to_image(self.normalmap, path)

    def _gen_heightmap(self, progress):
        bar = Bar('Generating heightmap', max=self.height)

        simplex = OpenSimplex()
        for y in range(self.height):
            for x in range(self.width):
                # Generate the height value for this pixel
                height = (simplex.noise2d(x/self.feature_size,
                    y/self.feature_size)+1) * 128

                # Save pixel height to map
                col = Color(height, height, height)
                self._set_px_color(self.heightmap, x, y, col)

            # Done with this row
            bar.next()

        bar.finish()

    def normalize(self, v):
        norm=np.linalg.norm(v, ord=1)
        if norm==0:
            norm=np.finfo(v.dtype).eps
        return v/norm

    def _calc_normal(self, x, y):
        """Calculates normal vector for a coordinate x,y on the heightmap.

        Math used is based on the following article:
        https://squircleart.github.io/shading/normal-map-generation.html
        """
        # Get the 3d coordinates of each point adjacent to x,y
        u = float(self.heightmap[(y+1) % self.height, x, 0])
        d = float(self.heightmap[(y-1) % self.height, x, 0])
        r = float(self.heightmap[y, (x+1) % self.width, 0])
        l = float(self.heightmap[y, (x-1) % self.width, 0])

        # Calc linear derivatives
        dzdx = (l - r)/2
        dzdy = (d - u)/2

        # Calculate normal
        norm = self.normalize(np.array([-dzdx, -dzdy, 1.0]))

        # Convert normals to colors so we can encode them as pixels
        norm = (norm+1) * 128

        return norm[0], norm[1], norm[2]

    def _gen_normalmap(self, progress):
        bar = Bar('Generating normalmap', max=self.height)

        for y in range(self.height):
            for x in range(self.width):
                nx, ny, nz = self._calc_normal(x, y)

                # Save pixel height to map
                col = Color(nx, ny, nz)
                self._set_px_color(self.normalmap, x, y, col)

            # Done with this row
            bar.next()

        bar.finish()

    def _set_px_color(self, data, x, y, color):
        data[y,x,0] = color.r
        data[y,x,1] = color.g
        data[y,x,2] = color.b

    def _get_px_color(self, data, x, y, color):
        return color.r, color.g, color.b

    def _numpy_to_image(self, data, path):
        img = Image.fromarray(data, 'RGB')
        img.save(path)

def main():
    h = 512
    m = MapGenerator(h, h*2, 100)
    m.generate()
    m.export_heightmap('heightmap.png')
    m.export_normalmap('normalmap.png')

if __name__ == '__main__':
    main()
