from kurbopy import Point, CubicBez
import math

def test_cubicbez_deriv():
    c = CubicBez(
        Point(0.0, 0.0),
        Point(1.0 / 3.0, 0.0),
        Point(2.0 / 3.0, 1.0 / 3.0),
        Point(1.0, 1.0),
    );
    deriv = c.deriv();

    n = 10;
    for i in range(1, n):
        t = 1/(i*n)
        delta = 1e-6
        p = c.eval(t)
        p1 = c.eval(t + delta)
        d_approx = (p1.to_vec2() - p.to_vec2()) * (1/delta)
        d = deriv.eval(t).to_vec2()
        assert (d - d_approx).hypot() < delta * 2.0



def test_cubicbez_arclen():
    # y = x^2
    c = CubicBez(
        Point(0.0, 0.0),
        Point(1.0 / 3.0, 0.0),
        Point(2.0 / 3.0, 1.0 / 3.0),
        Point(1.0, 1.0),
    );
    true_arclen = 0.5 * math.sqrt(5.0) + 0.25 * math.log(2.0 + math.sqrt(5.0))
    for i in range(0, 12):
        accuracy = 0.1 ** i
        error = c.arclen(accuracy) - true_arclen
        assert abs(error) < accuracy



# def test_cubicbez_inv_arclen():
#     // y = x^2 / 100
#     c = CubicBez(
#         Point(0.0, 0.0),
#         Point(100.0 / 3.0, 0.0),
#         Point(200.0 / 3.0, 100.0 / 3.0),
#         Point(100.0, 100.0),
#     );
#     true_arclen = 100.0 * (0.5 * 5.0f64.sqrt() + 0.25 * (2.0 + 5.0f64.sqrt()).ln());
#     for i in 0..12 {
#         accuracy = 0.1f64.powi(i);
#         n = 10;
#         for j in 0..=n {
#             arc = (j as f64) * ((n as f64).recip() * true_arclen);
#             t = c.inv_arclen(arc, accuracy * 0.5);
#             actual_arc = c.subsegment(0.0..t).arclen(accuracy * 0.5);
#             assert!(
#                 (arc - actual_arc).abs() < accuracy,
#                 "at accuracy {:e, wanted { got {",
#                 accuracy,
#                 actual_arc,
#                 arc
#             );
        
    
#     // corner case: user passes accuracy larger than total arc length
#     accuracy = true_arclen * 1.1;
#     arc = true_arclen * 0.5;
#     t = c.inv_arclen(arc, accuracy);
#     actual_arc = c.subsegment(0.0..t).arclen(accuracy);
#     assert!(
#         (arc - actual_arc).abs() < 2.0 * accuracy,
#         "at accuracy {:e, want { got {",
#         accuracy,
#         actual_arc,
#         arc
#     );



# def test_cubicbez_signed_area_linear():
#     #
#     c = CubicBez::new(
#         (1.0, 0.0),
#         (2.0 / 3.0, 1.0 / 3.0),
#         (1.0 / 3.0, 2.0 / 3.0),
#         (0.0, 1.0),
#     );
#     epsilon = 1e-12;
#     assert_eq!((Affine::rotate(0.5) * c).signed_area(), 0.5);
#     assert!(((Affine::rotate(0.5) * c).signed_area() - 0.5).abs() < epsilon);
#     assert!(((Affine::translate((0.0, 1.0)) * c).signed_area() - 1.0).abs() < epsilon);
#     assert!(((Affine::translate((1.0, 0.0)) * c).signed_area() - 1.0).abs() < epsilon);



# def test_cubicbez_signed_area():
#     // y = 1 - x^3
#     c = CubicBez::new((1.0, 0.0), (2.0 / 3.0, 1.0), (1.0 / 3.0, 1.0), (0.0, 1.0));
#     epsilon = 1e-12;
#     assert!((c.signed_area() - 0.75).abs() < epsilon);
#     assert!(((Affine::rotate(0.5) * c).signed_area() - 0.75).abs() < epsilon);
#     assert!(((Affine::translate((0.0, 1.0)) * c).signed_area() - 1.25).abs() < epsilon);
#     assert!(((Affine::translate((1.0, 0.0)) * c).signed_area() - 1.25).abs() < epsilon);



# def test_cubicbez_nearest():
#     fn verify(result: Nearest, expected: f64) {
#         assert!(
#             (result.t - expected).abs() < 1e-6,
#             "got {:? expected {",
#             result,
#             expected
#         );
    
#     // y = x^3
#     c = CubicBez::new((0.0, 0.0), (1.0 / 3.0, 0.0), (2.0 / 3.0, 0.0), (1.0, 1.0));
#     verify(c.nearest((0.1, 0.001).into(), 1e-6), 0.1);
#     verify(c.nearest((0.2, 0.008).into(), 1e-6), 0.2);
#     verify(c.nearest((0.3, 0.027).into(), 1e-6), 0.3);
#     verify(c.nearest((0.4, 0.064).into(), 1e-6), 0.4);
#     verify(c.nearest((0.5, 0.125).into(), 1e-6), 0.5);
#     verify(c.nearest((0.6, 0.216).into(), 1e-6), 0.6);
#     verify(c.nearest((0.7, 0.343).into(), 1e-6), 0.7);
#     verify(c.nearest((0.8, 0.512).into(), 1e-6), 0.8);
#     verify(c.nearest((0.9, 0.729).into(), 1e-6), 0.9);
#     verify(c.nearest((1.0, 1.0).into(), 1e-6), 1.0);
#     verify(c.nearest((1.1, 1.1).into(), 1e-6), 1.0);
#     verify(c.nearest((-0.1, 0.0).into(), 1e-6), 0.0);
#     a = Affine::rotate(0.5);
#     verify((a * c).nearest(a * Point::new(0.1, 0.001), 1e-6), 0.1);


# // ensure to_quads returns something given colinear points

# def test_degenerate_to_quads():
#     c = CubicBez::new((0., 9.), (6., 6.), (12., 3.0), (18., 0.0));
#     quads = c.to_quads(1e-6).collect::<Vec<_>>();
#     assert_eq!(quads.len(), 1, "{:?", &quads);



def test_cubicbez_extrema():
    q = CubicBez(Point(0.0, 0.0), Point(0.0, 1.0), Point(1.0, 1.0), Point(1.0, 0.0));
    extrema = q.extrema()
    assert len(extrema) == 1
    assert abs(extrema[0] - 0.5) < 1e-6

    q = CubicBez(Point(0.4, 0.5), Point(0.0, 1.0), Point(1.0, 0.0), Point(0.5, 0.4));
    extrema = q.extrema();
    assert len(extrema) == 4



# def test_cubicbez_toquads():
#     // y = x^3
#     c = CubicBez::new((0.0, 0.0), (1.0 / 3.0, 0.0), (2.0 / 3.0, 0.0), (1.0, 1.0));
#     for i in 0..10 {
#         accuracy = 0.1f64.powi(i);
#         mut worst: f64 = 0.0;
#         for (_count, (t0, t1, q)) in c.to_quads(accuracy).enumerate() {
#             epsilon = 1e-12;
#             assert!((q.start() - c.eval(t0)).hypot() < epsilon);
#             assert!((q.end() - c.eval(t1)).hypot() < epsilon);
#             n = 4;
#             for j in 0..=n {
#                 t = (j as f64) * (n as f64).recip();
#                 p = q.eval(t);
#                 err = (p.y - p.x.powi(3)).abs();
#                 worst = worst.max(err);
#                 assert!(err < accuracy, "got { wanted {", err, accuracy);
            
        
    

