from .kurbopy import Point as Point
from .kurbopy import Rect as Rect
from .kurbopy import Vec2 as Vec2
from .kurbopy import Line as Line
from .kurbopy import CubicBez as CubicBez
from .kurbopy import QuadBez as QuadBez
from .kurbopy import TranslateScale as TranslateScale
from .kurbopy import BezPath as BezPath
from fontTools.pens.basePen import BasePen
from kurbopy.magic import magic_mul, magic_add, magic_sub


def fromDrawable(drawable, *penArgs, **penKwargs):
    """Returns an *array of BezPath* from any object conforming to the pen protocol."""
    pen = BezPathCreatingPen(*penArgs, **penKwargs)
    drawable.draw(pen)
    return pen.paths

setattr(BezPath, "fromDrawable", fromDrawable)


class BezPathCreatingPen(BasePen):
    def __init__(self, *args, **kwargs):
        super(BezPathCreatingPen, self).__init__(*args, **kwargs)
        self.paths = []
        self.path = BezPath()

    def _moveTo(self, p):
        self.path.move_to(Point(p[0], p[1]))

    def _lineTo(self, p):
        self.path.line_to(Point(p[0], p[1]))

    def _curveToOne(self, p1, p2, p3):
        self.path.curve_to(
            Point(p1[0], p1[1]), Point(p2[0], p2[1]), Point(p3[0], p3[1])
        )

    def _qCurveToOne(self, p1, p2):
        self.path.quad_to(Point(p1[0], p1[1]), Point(p2[0], p2[1]))

    def _closePath(self):
        self.path.close_path()
        self.paths.append(self.path)
        self.path = BezPath()
