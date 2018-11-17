from .base import Map
from .color import Color

class HeightMap(Map):
    def __init__(self, width, height, feature_size=50.0):
        super().__init__(width, height)
        self.map_noise(float(feature_size))
