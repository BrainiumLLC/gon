use crate::{
    options::{Options, StrokeOptions},
    tess,
    vertex::{FillVertexConstructor, StrokeVertexConstructor, Vertex},
    Poly, PolyBuilder,
};

#[derive(Clone, Debug, Default)]
pub struct FreePolyBuilder {
    points: Vec<gee::Point<f32>>,
    open: bool,
    bounding_box: Option<gee::Rect<f32>>,
    options: Options,
}

impl FreePolyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_points(points: impl IntoIterator<Item = gee::Point<f32>>) -> Self {
        Self::default().with_points(points)
    }

    pub fn with_point(mut self, point: gee::Point<f32>) -> Self {
        self.points.push(point);
        self.bounding_box = self
            .bounding_box
            .map(|bounding_box| bounding_box.grow_to(point))
            .or_else(|| {
                if self.points.len() == 2 {
                    Some(gee::Rect::from_points(self.points[0], self.points[1]))
                } else {
                    None
                }
            });
        self
    }

    pub fn with_points(self, points: impl IntoIterator<Item = gee::Point<f32>>) -> Self {
        points
            .into_iter()
            .fold(self, |this, point| this.with_point(point))
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

    pub fn try_build(self) -> Result<Poly, tess::TessellationError> {
        crate::try_build(self)
    }

    pub fn build(self) -> Poly {
        crate::build(self)
    }
}

impl PolyBuilder for FreePolyBuilder {
    fn build_in_place(
        self,
        vertex_buffers: &mut tess::VertexBuffers<Vertex, u32>,
    ) -> Result<(), tess::TessellationError> {
        let _count = match self.options.stroke_options.clone() {
            None => tess::basic_shapes::fill_convex_polyline(
                self.points.into_iter().map(crate::point),
                &self.options.fill_options(),
                &mut tess::BuffersBuilder::new(
                    vertex_buffers,
                    FillVertexConstructor::new(self.bounding_box.unwrap_or_default()),
                ),
            )?,
            Some(stroke_options) => tess::basic_shapes::stroke_polyline(
                self.points.into_iter().map(crate::point),
                !self.open, // closed flag
                &self.options.stroke_options(),
                &mut tess::BuffersBuilder::new(
                    vertex_buffers,
                    StrokeVertexConstructor::new(
                        stroke_options.stroke_width,
                        stroke_options.texture_aspect_ratio,
                    ),
                ),
            )?,
        };
        Ok(())
    }
}
