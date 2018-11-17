from .base import Map, Color
from .color import Color

SEA_LEVEL = 0.3
BEACH_LEVEL = 0.35
LAND_LEVEL = 0.6
FOREST_LEVEL = 0.8
MOUNTAIN_LEVEL = 0.85
CLIFF_LEVEL = 0.95
SNOW_LEVEL = 1.0

class TopographicMap(Map):
    def __init__(self, width, height):
        super().__init__(width, height)

    def biome_sea_filter(self, x, y, height):
        if height >= SEA_LEVEL - 0.5:
            return Color(0, 0, 200)
        else:
            return Color(0, 0, 30)

    def biome_land_filter(self, x, y, height):
        if height < BEACH_LEVEL:
            return Color(194, 178, 128)
        if height < LAND_LEVEL:
            return Color(135, 169, 107)
        if height < FOREST_LEVEL:
            return Color(75, 111, 68)
        if height < MOUNTAIN_LEVEL:
            return Color(144, 108, 63)
        if height < CLIFF_LEVEL:
            return Color(100)
        if height < SNOW_LEVEL:
            return Color(220)

    def biome_filter(self, x, y, height):
        # TODO: create biome class, create mechanism to look up biomes based on pixel color so we can render special properties into them
        if height < SEA_LEVEL:
            return self.biome_sea_filter(x, y, height)
        else:
            return self.biome_land_filter(x, y, height)

    @classmethod
    def from_heightmap(cls, heightmap):
        c = cls(heightmap.width, heightmap.height)

        def calc_biome(x, y):
            height = heightmap.get_col(x, y).r / 255
            return c.biome_filter(x, y, height)
            
        c.map_coords(calc_biome)

        return c
