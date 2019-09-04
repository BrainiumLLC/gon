use crate::{
    options::{Options, Tessellation},
    tess,
    vertex::{FillVertexConstructor, StrokeVertexConstructor, Vertex},
    Poly, PolyBuilder,
};

#[derive(Clone, Debug, Default)]
pub struct CircleBuilder {
    pub circle: gee::Circle<f32>,
    pub options: Options,
}

impl CircleBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_center(mut self, center: gee::Point<f32>) -> Self {
        self.circle.center = center;
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.circle.radius = radius;
        self
    }

    options_forwarder! {}

    pub fn try_build(self) -> Result<Poly, tess::TessellationError> {
        crate::try_build(self)
    }

    pub fn build(self) -> Poly {
        crate::build(self)
    }
}

impl PolyBuilder for CircleBuilder {
    fn build_in_place(
        self,
        vertex_buffers: &mut tess::VertexBuffers<Vertex, u32>,
    ) -> Result<(), tess::TessellationError> {
        let _count = match self.options.tessellation {
            Tessellation::Fill => tess::basic_shapes::fill_circle(
                crate::point(self.circle.center),
                self.circle.radius,
                &self.options.fill_options(),
                &mut tess::BuffersBuilder::new(
                    vertex_buffers,
                    FillVertexConstructor::new(
                        self.options.color,
                        self.circle.bounding_rect(),
                        self.options.texture_aspect_ratio,
                    ),
                ),
            )?,
            Tessellation::Stroke => tess::basic_shapes::stroke_circle(
                crate::point(self.circle.center),
                self.circle.radius,
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
