use crate::{
    options::{Options, StrokeOptions},
    tess, PolyBuilder,
};
use gee::{Point, Rect};

#[derive(Clone, Debug)]
pub enum ControlPoint {
    Quadratic(Point),
    Cubic(Point, Point),
}

#[derive(Clone, Debug)]
pub struct BezierSegment {
    end: Point,
    ctrl: ControlPoint,
}

impl BezierSegment {
    pub fn new(end: Point, ctrl: ControlPoint) -> Self {
        Self { end, ctrl }
    }

    pub fn quadratic(end: Point, ctrl: Point) -> Self {
        Self::new(end, ControlPoint::Quadratic(ctrl))
    }

    pub fn cubic(end: Point, ctrl1: Point, ctrl2: Point) -> Self {
        Self::new(end, ControlPoint::Cubic(ctrl1, ctrl2))
    }
}

#[derive(Clone, Debug, Default)]
pub struct BezierBuilder {
    start: Point,
    segments: Vec<BezierSegment>,
    open: bool,
    options: Options,
}

impl BezierBuilder {
    pub fn new(start: Point) -> Self {
        Self::default().with_start(start)
    }

    pub fn from_bezier_segment(start: Point, segment: BezierSegment) -> Self {
        Self::new(start).with_bezier_segment(segment)
    }

    pub fn from_bezier_segments(
        start: Point,
        segments: impl IntoIterator<Item = BezierSegment>,
    ) -> Self {
        Self::new(start).with_bezier_segments(segments)
    }

    pub fn from_quadratic_segment(start: Point, end: Point, ctrl: Point) -> Self {
        Self::from_bezier_segment(start, BezierSegment::quadratic(end, ctrl))
    }

    pub fn from_cubic_segment(start: Point, end: Point, ctrl1: Point, ctrl2: Point) -> Self {
        Self::from_bezier_segment(start, BezierSegment::cubic(end, ctrl1, ctrl2))
    }

    pub fn with_start(mut self, start: Point) -> Self {
        self.start = start;
        self
    }

    pub fn with_bezier_segment(mut self, segment: BezierSegment) -> Self {
        self.segments.push(segment);
        self
    }

    pub fn with_bezier_segments(
        mut self,
        segments: impl IntoIterator<Item = BezierSegment>,
    ) -> Self {
        self.segments.extend(segments);
        self
    }

    pub fn with_quadratic_segment(self, end: Point, ctrl: Point) -> Self {
        self.with_bezier_segment(BezierSegment::quadratic(end, ctrl))
    }

    pub fn with_cubic_segment(self, end: Point, ctrl1: Point, ctrl2: Point) -> Self {
        self.with_bezier_segment(BezierSegment::cubic(end, ctrl1, ctrl2))
    }

    pub fn with_stroke(mut self, stroke_width: f32, open: bool) -> Self {
        self.open = open;
        self._with_stroke(stroke_width)
    }

    pub fn with_stroke_open(self, stroke_width: f32) -> Self {
        self.with_stroke(stroke_width, true)
    }

    pub fn with_stroke_closed(self, stroke_width: f32) -> Self {
        self.with_stroke(stroke_width, false)
    }

    pub fn with_stroke_opts(mut self, stroke_options: StrokeOptions, open: bool) -> Self {
        self.open = open;
        self._with_stroke_opts(stroke_options)
    }

    pub fn with_stroke_opts_open(self, stroke_options: StrokeOptions) -> Self {
        self.with_stroke_opts(stroke_options, true)
    }

    pub fn with_stroke_opts_closed(self, stroke_options: StrokeOptions) -> Self {
        self.with_stroke_opts(stroke_options, false)
    }

    stroke!(private);

    fill!();

    build!();
}

impl PolyBuilder for BezierBuilder {
    fn options(&self) -> &Options {
        &self.options
    }

    fn bounding_rect(&self) -> Rect {
        todo!("bézier curves can't be filled")
    }

    fn build<B: tess::path::traits::PathBuilder>(self, builder: &mut B) {
        builder.begin(self.start.into());
        for segment in self.segments {
            match segment.ctrl {
                ControlPoint::Quadratic(ctrl) => {
                    builder.quadratic_bezier_to(ctrl.into(), segment.end.into());
                }
                ControlPoint::Cubic(ctrl1, ctrl2) => {
                    builder.cubic_bezier_to(ctrl1.into(), ctrl2.into(), segment.end.into());
                }
            }
        }
        builder.end(!self.open);
    }
}
