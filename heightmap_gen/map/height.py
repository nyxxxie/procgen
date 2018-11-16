from .base import Map
from .color import Color

class HeightMap(Map):
    def __init__(self, width, height):
        super().__init__(width, height)
        self.map_noise(50.0)
