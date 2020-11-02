import json
import random
import math

TWO_WEEKS = 2070000000
START_TIME = 1600129528950


def randomGeo(center, radius):
    y0 = center['latitude']
    x0 = center['longitude']
    rd = radius / 111300

    u = random.random()
    v = random.random()

    w = rd * math.sqrt(u)
    t = 2 * math.pi * v
    x = w * math.cos(t)
    y = w * math.sin(t)

    timestamp = random.randrange(START_TIME, START_TIME + TWO_WEEKS)

    return {'latitudeE7': int((y + y0) * 1e7),
            'longitudeE7': int((x + x0) * 1e7),
            'timestampMs': str(timestamp)}


def do():
    big = 100000

    center = {'latitude': 52.5200, 'longitude': 13.4050}
    for i in range(100):
        pts = []
        for _ in range(big):
            pts.append(randomGeo(center, 10000))

        msg = {'import_google_locations': {'data': {"locations": pts}}}

        with open(f"data/points{i}.json", "+w") as f:
            f.write(json.dumps(msg))


if __name__ == '__main__':
    do()
