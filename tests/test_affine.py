from kurbopy import Affine


def test_affine_mul():
    i = Affine.IDENTITY()
    a2 = 2.0 * i
    assert a2.as_coeffs()[0] == 2.0
