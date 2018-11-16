import numpy as np
from PIL import Image
from opensimplex import OpenSimplex
from .color import Color

class Map(object):
    def __init__(self, width, height):
        self.width = width
        self.height = height
        self.map_data = np.zeros((height, width, 3), dtype=np.uint8)

    def set_col(self, x, y, color):
        """."""
        # TODO: figure out how to support images with multiple color types
        self.map_data[y,x,0] = color.r
        self.map_data[y,x,1] = color.g
        self.map_data[y,x,2] = color.b

    def get_col(self, x, y):
        """."""
        # TODO: figure out how to support images with multiple color types
        return Color(self.map_data[y,x,0], self.map_data[y,x,1],
                self.map_data[y,x,2])

    def map_coords(self, fn):
        for y in range(self.height):
            for x in range(self.width):
                self.set_col(x, y, fn(x, y))

    def to_image(self, path):
        img = Image.fromarray(self.map_data, 'RGB')
        img.save(path)

    def map_noise(self, fsize):
        s = OpenSimplex()
        self.map_coords(lambda x,y: Color((s.noise2d(x/fsize, y/fsize)+1) * 128))

    @classmethod
    def from_image(cls, path):
        pass
