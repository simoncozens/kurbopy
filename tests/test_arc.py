import math
from kurbopy import Arc, Point, Vec2


def test_arc_iterator():
    arc = Arc(Point(1, 1), Vec2(1, 0), math.pi / 2, math.pi, 0)
    lst = []
    arc.to_cubic_beziers(0.5, lambda a, b, c: lst.append((a, b, c)))
    assert len(lst) == 2
