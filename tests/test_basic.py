from kurbopy import Point, Vec2, TranslateScale
import pytest


def test_point_basic():
    p = Point(1.0, 2.0)
    assert p.x == 1.0
    p.y = 3.0
    assert p.y == 3.0
    assert str(p) == "<Point x=1.0 y=3.0>"


def test_point_magic():
    p = Point(1.0, 2.0) + Vec2(1.0, 1.0)
    assert p.x == 2.0
    p = Point(1.0, 2.0) + (1.0, 1.0)
    assert p.x == 2.0


def test_point_lerp():
    p1 = Point(0.0, 0.0)
    p2 = Point(1.0, 2.0)
    mid = p1.lerp(p2, 0.5)
    assert mid.x == 0.5
    assert mid.y == 1.0


def test_point_round():
    p = Point(1.2, 2.0)
    assert p.round().x == 1.0


def test_vec2_protocol():
    p1 = Vec2(1.2, 2.0)
    p2 = Vec2(3.0, 4.0)
    assert (p1 + p2).x == 4.2
    p1 += p2
    assert p1.x == 4.2
    p1 *= 2
    assert p1.x == 8.4
    with pytest.raises(TypeError):
        p1 *= p2


def test_ts():
    ts1 = TranslateScale(Vec2(3.0, 2.0), 1.0)
    ts2 = TranslateScale(Vec2(3.0, 2.0), 1.0)
    p1 = Point(10.0, 10.0)
    moved = ts1 * p1
    assert moved.x == 13.0
    assert moved.y == 12.0


def test_common():
    from kurbopy.common import solve_cubic

    assert solve_cubic(0.1, 0.1, 0.5, 0.2) == [-2.378160678793357]

    from kurbopy.common import solve_itp

    f = lambda x: x**3 - x - 2.0
    x = solve_itp(f, 1.0, 2.0, 1e-12, 0, 0.2, f(1.0), f(2.0))
    assert abs(f(x)) < 6e-12
