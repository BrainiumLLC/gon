use crate::{
    options::{Options, StrokeOptions},
    tess, PolyBuilder, DEFAULT_RADIUS,
};
use gee::{LineSegment, Point, Rect};

#[derive(Clone, Debug)]
pub struct LineSegmentBuilder {
    pub line: LineSegment,
    pub options: Options,
}

impl Default for LineSegmentBuilder {
    fn default() -> Self {
        Self {
            line: LineSegment::new(Point::new(0.0, 0.0), Point::new(DEFAULT_RADIUS * 2.0, 0.0)),
            options: Options::default().with_stroke(Default::default()),
        }
    }
}

impl LineSegmentBuilder {
    pub fn new(line: LineSegment) -> Self {
        Self::default().with_line_segment(line)
    }

    pub fn with_line_segment(mut self, line: LineSegment) -> Self {
        self.line = line;
        self
    }

    stroke!(public);

    build!();
}

impl PolyBuilder for LineSegmentBuilder {
    fn options(&self) -> &Options {
        &self.options
    }

    fn bounding_rect(&self) -> Rect {
        todo!("line segments can't be filled")
    }

    fn build<B: tess::path::traits::PathBuilder>(self, builder: &mut B) {
        builder.add_line_segment(&tess::geom::LineSegment {
            from: self.line.from.into(),
            to: self.line.to.into(),
        });
    }
}
