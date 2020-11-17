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

    return {'latitudeE7': int((y + y0) * 10e7),
            'longitudeE7': int((x + x0) * 10e7),
            'timestampMs': str(timestamp)}


def do():
    big = 3000000
    pts = []
    center = {'latitude': 13.4050, 'longitude': 52.5200}
    for _ in range(big):
        pts.append(randomGeo(center, 10000))

    with open("points4.json", "+w") as f:
        f.write(json.dumps(pts))


if __name__ == '__main__':
    do()