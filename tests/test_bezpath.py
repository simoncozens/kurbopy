from kurbopy import Point, BezPath
import math


def test_bezpath_segments():
    b = BezPath()
    b.move_to(Point(0, 0))
    b.line_to(Point(100, 100))
    b.line_to(Point(100, 0))
    b.close_path()
    assert len(b.segments()) == 3
