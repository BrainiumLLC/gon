use crate::{
    options::{Options, StrokeOptions},
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
    pub fn new(circle: gee::Circle<f32>) -> Self {
        Self::default().with_circle(circle)
    }

    pub fn with_circle(mut self, circle: gee::Circle<f32>) -> Self {
        self.circle = circle;
        self
    }

    stroke!(public);

    fill!();

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
                crate::point(self.circle.center()),
                self.circle.radius(),
                &self.options.fill_options(),
                &mut tess::BuffersBuilder::new(
                    vertex_buffers,
                    FillVertexConstructor::new(self.circle.bounding_rect()),
                ),
            )?,
            Some(stroke_options) => tess::basic_shapes::stroke_circle(
                crate::point(self.circle.center()),
                self.circle.radius(),
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
