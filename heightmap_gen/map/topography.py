from .base import Map, Color
from .color import Color

class TopographicMap(Map):
    def __init__(self, width, height):
        super().__init__(width, height)

    def biome_filter(self, x, y, height):
        if height > 0.5:
            return Color(255, 0, 0)
        else:
            return Color(0, 0, 0)

    @classmethod
    def from_heightmap(cls, heightmap):
        c = cls(heightmap.width, heightmap.height)

        def calc_biome(x, y):
            height = heightmap.get_col(x, y).r / 255
            return c.biome_filter(x, y, height)
            
        c.map_coords(calc_biome)

        return c
