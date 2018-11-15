#!/usr/bin/env python
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

    def generate(self, progress=True):
        self._gen_heightmap(progress)

    def export_heightmap(self, path):
        self._numpy_to_image(self.heightmap, path)

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

    def _set_px_color(self, data, x, y, color):
        data[y,x,0] = color.r
        data[y,x,1] = color.g
        data[y,x,2] = color.b

    def _numpy_to_image(self, data, path):
        img = Image.fromarray(data, 'RGB')
        img.save(path)

def main():
    h = 2048
    m = MapGenerator(h, h*2, 30.0)
    m.generate()
    m.export_heightmap('heightmap.png')

if __name__ == '__main__':
    main()
