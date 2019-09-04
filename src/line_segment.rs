use crate::{
    options::{Options, Tessellation},
    tess,
    vertex::{StrokeVertexConstructor, Vertex},
    Poly, PolyBuilder,
};

#[derive(Clone, Debug)]
pub struct LineSegmentBuilder {
    pub line: gee::LineSegment<f32>,
    pub options: Options,
}

impl Default for LineSegmentBuilder {
    fn default() -> Self {
        Self {
            line: gee::LineSegment::new(gee::Point::new(0.0, 0.0), gee::Point::new(1.0, 0.0)),
            options: Options::default().with_stroke(),
        }
    }
}

impl LineSegmentBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_line_segment(mut self, line: gee::LineSegment<f32>) -> Self {
        self.line = line;
        self
    }

    options_forwarder! {fixed_tessellation}

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
        let _count = match self.options.tessellation {
            Tessellation::Fill => panic!("cannot Tessellate a Line using `Fill`"),
            Tessellation::Stroke => tess::basic_shapes::stroke_polyline(
                [crate::point(self.line.from), crate::point(self.line.to)]
                    .iter()
                    .cloned(),
                true, // closed
                &self.options.stroke_options(),
                &mut tess::BuffersBuilder::new(
                    vertex_buffers,
                    StrokeVertexConstructor::new(
                        self.options.color,
                        self.options.stroke_width,
                        self.options.texture_aspect_ratio,
                    ),
                ),
            )?,
        };
        Ok(())
    }
}
