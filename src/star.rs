use crate::{
    default_start_angle,
    options::{Options, StrokeOptions},
    tess,
    vertex::{FillVertexConstructor, StrokeVertexConstructor, Vertex},
    Poly, PolyBuilder, DEFAULT_RADIUS,
};
use itertools::Itertools as _;

#[derive(Clone, Debug)]
pub struct StarBuilder {
    circle: gee::Circle<f32>,
    inner_radius_over_radius: f32,
    tips: u32,
    start_angle: gee::Angle<f32>,
    options: Options,
}

impl Default for StarBuilder {
    fn default() -> Self {
        Self {
            circle: gee::Circle::with_radius(DEFAULT_RADIUS),
            inner_radius_over_radius: 0.5,
            tips: 5,
            start_angle: default_start_angle(),
            options: Default::default(),
        }
    }
}

impl StarBuilder {
    pub fn new(tips: u32) -> Self {
        Self::default().with_tips(tips)
    }

    pub fn pentagram() -> Self {
        Self::new(5)
    }

    pub fn hexagram() -> Self {
        Self::new(6)
    }

    pub fn with_tips(mut self, tips: u32) -> Self {
        assert!(
            tips >= 3,
            "stars must have at least 3 tips, but this one has {}",
            tips
        );
        self.tips = tips;
        self
    }

    pub fn with_circle(mut self, circle: gee::Circle<f32>) -> Self {
        self.circle = circle;
        self
    }

    pub fn with_rotation(mut self, start_angle: impl Into<gee::Angle<f32>>) -> Self {
        self.start_angle = start_angle.into();
        self
    }

    /// The lower this value, the more pointy the star is.
    ///
    /// Values for `inner_radius_over_radius` must be in the range (0, 1].
    pub fn with_inner_radius_ratio(mut self, inner_radius_over_radius: f32) -> Self {
        assert!(
            inner_radius_over_radius > 0.0 && inner_radius_over_radius <= 1.0,
            "`inner_radius_ratio` must be in the range `(0, 1]`"
        );
        self.inner_radius_over_radius = inner_radius_over_radius;
        self
    }

    options_forwarder! {}

    fn points(&self) -> impl Iterator<Item = tess::math::Point> + Clone {
        let top_angle = self.start_angle;
        let inner_offset = gee::Angle::PI() / self.tips as f32;
        let inner_circle = self.circle.scaled_radius(self.inner_radius_over_radius);
        self.circle
            .circle_points(self.tips, top_angle)
            .interleave(inner_circle.circle_points(self.tips, top_angle + inner_offset))
            .map(crate::point)
    }

    pub fn try_build(self) -> Result<Poly, tess::TessellationError> {
        crate::try_build(self)
    }

    pub fn build(self) -> Poly {
        crate::build(self)
    }
}

impl PolyBuilder for StarBuilder {
    fn build_in_place(
        self,
        vertex_buffers: &mut tess::VertexBuffers<Vertex, u32>,
    ) -> Result<(), tess::TessellationError> {
        let _count = match self.options.stroke_options.clone() {
            None => tess::basic_shapes::fill_convex_polyline(
                self.points(),
                &self.options.fill_options(),
                &mut tess::BuffersBuilder::new(
                    vertex_buffers,
                    FillVertexConstructor::new(self.circle.bounding_rect()),
                ),
            )?,
            Some(stroke_options) => tess::basic_shapes::stroke_polyline(
                self.points(),
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
