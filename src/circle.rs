use crate::{
    options::{Options, StrokeOptions},
    tess,
    vertex::{FillVertexConstructor, StrokeVertexConstructor, Vertex},
    Poly, PolyBuilder, DEFAULT_RADIUS,
};

#[derive(Clone, Debug, Default)]
pub struct CircleBuilder {
    pub circle: gee::Circle<f32>,
    pub options: Options,
}

impl CircleBuilder {
    pub fn new() -> Self {
        Self {
            circle: gee::Circle {
                radius: DEFAULT_RADIUS,
                ..Default::default()
            },
            options: Default::default(),
        }
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
        let _count = match self.options.stroke_options.clone() {
            None => tess::basic_shapes::fill_circle(
                crate::point(self.circle.center),
                self.circle.radius,
                &self.options.fill_options(),
                &mut tess::BuffersBuilder::new(
                    vertex_buffers,
                    FillVertexConstructor::new(self.options.color, self.circle.bounding_rect()),
                ),
            )?,
            Some(stroke_options) => tess::basic_shapes::stroke_circle(
                crate::point(self.circle.center),
                self.circle.radius,
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
