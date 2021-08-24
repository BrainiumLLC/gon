use crate::{
    options::{Options, StrokeOptions},
    tess, PolyBuilder,
};
use gee::{LineSegment, Point, Rect};

#[derive(Clone, Debug, Default)]
pub struct FreePolyBuilder {
    points: Vec<tess::geom::Point<f32>>,
    open: bool,
    bounding_rect: Option<Rect>,
    options: Options,
}

impl FreePolyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn from_parts(
        points: impl IntoIterator<Item = Point>,
        open: bool,
        bounding_rect: Option<Rect>,
        options: Options,
    ) -> Self {
        Self {
            open,
            bounding_rect,
            options,
            ..Self::from_points(points)
        }
    }

    pub fn from_points(points: impl IntoIterator<Item = Point>) -> Self {
        Self::default().with_points(points.into_iter().map(Into::into))
    }

    pub fn from_line_segments(lines: impl IntoIterator<Item = LineSegment>) -> Self {
        Self::default().with_line_segments(lines)
    }

    pub fn with_point(mut self, point: Point) -> Self {
        self.points.push(point.into());
        self.bounding_rect = self
            .bounding_rect
            .map(|bounding_rect| bounding_rect.grow_to(point))
            .or_else(|| {
                if self.points.len() == 2 {
                    Some(Rect::from_points(
                        self.points[0].into(),
                        self.points[1].into(),
                    ))
                } else {
                    None
                }
            });
        self
    }

    pub fn with_points(self, points: impl IntoIterator<Item = Point>) -> Self {
        points
            .into_iter()
            .fold(self, |this, point| this.with_point(point))
    }

    pub fn with_line_segment(self, line: LineSegment) -> Self {
        self.with_points(line.points())
    }

    pub fn with_line_segments(self, lines: impl IntoIterator<Item = LineSegment>) -> Self {
        self.with_points(lines.into_iter().flat_map(|line| line.points()))
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

impl PolyBuilder for FreePolyBuilder {
    fn options(&self) -> &Options {
        &self.options
    }

    fn bounding_rect(&self) -> Rect {
        self.bounding_rect.unwrap_or_default()
    }

    fn build<B: tess::path::traits::PathBuilder>(self, builder: &mut B) {
        builder.add_polygon(tess::path::Polygon {
            points: &self.points,
            closed: !self.open,
        });
    }
}
