use crate::{point, OutlineCurve, Point};
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[derive(Debug, Default)]
pub(crate) struct OutlineCurveBuilder {
    last: Point,
    last_move: Option<Point>,
    outline: Vec<OutlineCurve>,
}

impl OutlineCurveBuilder {
    #[inline]
    pub(crate) fn take_outline(self) -> Vec<OutlineCurve> {
        self.outline
    }
}

impl owned_ttf_parser::OutlineBuilder for OutlineCurveBuilder {
    #[inline]
    fn move_to(&mut self, x: f32, y: f32) {
        self.last = point(x, y);
        self.last_move = Some(self.last);
    }

    #[inline]
    fn line_to(&mut self, x1: f32, y1: f32) {
        let p1 = point(x1, y1);
        self.outline.push(OutlineCurve::Line(self.last, p1));
        self.last = p1;
    }

    #[inline]
    fn quad_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let p1 = point(x1, y1);
        let p2 = point(x2, y2);
        self.outline.push(OutlineCurve::Quad(self.last, p1, p2));
        self.last = p2;
    }

    #[inline]
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        let p1 = point(x1, y1);
        let p2 = point(x2, y2);
        let p3 = point(x3, y3);

        self.outline
            .push(OutlineCurve::Cubic(self.last, p1, p2, p3));
        self.last = p3;
    }

    #[inline]
    fn close(&mut self) {
        if let Some(m) = self.last_move {
            self.outline.push(OutlineCurve::Line(self.last, m));
        }
    }
}
