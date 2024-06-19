from kurbopy import Rect, Point, Size, Insets


def test_positive_insets():
    rect = Rect.from_origin_size(
        Point(
            0.0,
            0.0,
        ),
        Size(
            10.0,
            10.0,
        ),
    )
    insets = Insets.uniform_xy(3.0, 0.0)
    inset_rect = rect + insets
    assert inset_rect.width() == 16.0, "10.0 + 3.0 x 2"
    assert inset_rect.x0 == -3.0


def test_negative_insets():
    rect = Rect.from_origin_size(
        Point(
            0.0,
            0.0,
        ),
        Size(
            10.0,
            10.0,
        ),
    )
    insets = Insets.uniform_xy(-3.0, 0.0)
    inset_rect = rect + insets
    assert inset_rect.width() == 4.0, "10.0 - 3.0 × 2"
    assert inset_rect.x0 == 3.0


def test_absolute():
    rect = Rect(7.0, 11.0, 0.0, 0.0)
    insets = Insets.uniform_xy(0.0, 1.0)
    assert rect.width() == -7.0
    inset_rect = rect + insets
    assert inset_rect.width() == 7.0
    assert inset_rect.x0 == 0.0
    assert inset_rect.height() == 13.0


def test_rect_rect():
    rect = Rect(0.0, 0.0, 5.0, 11.0)
    insets = Insets.uniform_xy(
        1.0,
        7.0,
    )
    inset_rect = rect + insets
    insets2 = inset_rect - rect

    assert insets2.x0 == insets.x0
    assert insets2.y1 == insets.y1
    assert insets2.x_value() == insets.x_value()
    assert insets2.y_value() == insets.y_value()
