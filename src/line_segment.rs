use crate::{
    options::{Options, StrokeOptions},
    tess,
    vertex::{StrokeVertexConstructor, Vertex},
    Poly, PolyBuilder, DEFAULT_RADIUS,
};

#[derive(Clone, Debug)]
pub struct LineSegmentBuilder {
    pub line: gee::LineSegment<f32>,
    pub options: Options,
}

impl Default for LineSegmentBuilder {
    fn default() -> Self {
        Self {
            line: gee::LineSegment::new(
                gee::Point::new(0.0, 0.0),
                gee::Point::new(DEFAULT_RADIUS * 2.0, 0.0),
            ),
            options: Options::default().stroke(Default::default()),
        }
    }
}

impl LineSegmentBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn line_segment(mut self, line: gee::LineSegment<f32>) -> Self {
        self.line = line;
        self
    }

    options_forwarder! {no_fill}

    pub fn try_build(self) -> Result<Poly, tess::TessellationError> {
        crate::try_build(self)
    }

    pub fn build(self) -> Poly {
        crate::build(self)
    }
}

impl PolyBuilder for LineSegmentBuilder {
    fn build_in_place(
        self,
        vertex_buffers: &mut tess::VertexBuffers<Vertex, u32>,
    ) -> Result<(), tess::TessellationError> {
        let _count = match self.options.stroke_options.clone() {
            None => panic!("cannot Tessellate a Line using `Fill`"),
            Some(stroke_options) => tess::basic_shapes::stroke_polyline(
                [crate::point(self.line.from), crate::point(self.line.to)]
                    .iter()
                    .cloned(),
                true, // closed
                &self.options.stroke_options(),
                &mut tess::BuffersBuilder::new(
                    vertex_buffers,
                    StrokeVertexConstructor::new(
                        self.options.color,
                        stroke_options.stroke_width,
                        stroke_options.texture_aspect_ratio,
                    ),
                ),
            )?,
        };
        Ok(())
    }
}
