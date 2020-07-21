use crate::{
    options::{Options, StrokeOptions},
    tess,
    vertex::{FillVertexConstructor, StrokeVertexConstructor, Vertex},
    Poly, PolyBuilder,
};

pub const DEFAULT_RADIUS: f32 = 30.0;

#[derive(Clone, Debug)]
pub struct RoundRectBuilder {
    rect: gee::Rect<f32>,
    radius: f32,
    steps_per_radius: f32,
    options: Options,
}

impl Default for RoundRectBuilder {
    fn default() -> Self {
        Self {
            rect: gee::Rect::with_bottom_left(
                gee::Point::zero(),
                gee::Size::square(DEFAULT_RADIUS * 3.0),
            ),
            radius: DEFAULT_RADIUS,
            steps_per_radius: 1.0,
            options: Default::default(),
        }
    }
}

impl RoundRectBuilder {
    pub fn new(rect: gee::Rect<f32>) -> Self {
        Self::default().with_rect(rect)
    }

    pub fn with_rect(mut self, rect: gee::Rect<f32>) -> Self {
        self.rect = rect;
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn with_steps_per_radius(mut self, steps_per_radius: f32) -> Self {
        self.steps_per_radius = steps_per_radius;
        self
    }

    options_forwarder! {}

    fn points(&self) -> Vec<tess::math::Point> {
        let steps = (self.steps_per_radius * self.radius).round() as u32;
        self.rect
            .padded_uniform(self.radius)
            .clockwise_points()
            .rev()
            .zip(
                std::iter::once(gee::Cardinal::West.angle())
                    .chain(std::iter::once(gee::Cardinal::South.angle()))
                    .chain(std::iter::once(gee::Cardinal::East.angle()))
                    .chain(std::iter::once(gee::Cardinal::North.angle())),
            )
            .flat_map(|(center, start_angle)| {
                gee::Circle::new(center, self.radius).arc_points(
                    steps,
                    start_angle,
                    start_angle + gee::Angle::FRAC_PI_2(),
                )
            })
            .map(crate::point)
            .collect()
    }

    pub fn try_build(self) -> Result<Poly, tess::TessellationError> {
        crate::try_build(self)
    }

    pub fn build(self) -> Poly {
        crate::build(self)
    }
}

impl PolyBuilder for RoundRectBuilder {
    fn build_in_place(
        self,
        vertex_buffers: &mut tess::VertexBuffers<Vertex, u32>,
    ) -> Result<(), tess::TessellationError> {
        let _count = match self.options.stroke_options.clone() {
            None => tess::basic_shapes::fill_convex_polyline(
                self.points().into_iter(),
                &self.options.fill_options(),
                &mut tess::BuffersBuilder::new(
                    vertex_buffers,
                    FillVertexConstructor::new(self.rect),
                ),
            )?,
            Some(stroke_options) => tess::basic_shapes::stroke_polyline(
                self.points().into_iter(),
                true, // closed
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
