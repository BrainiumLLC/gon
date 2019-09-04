use crate::{
    options::{Options, Tessellation},
    tess,
    vertex::{FillVertexConstructor, StrokeVertexConstructor, Vertex},
    Poly, PolyBuilder,
};
use itertools::Itertools as _;

#[derive(Clone, Debug)]
pub struct RegularStarBuilder {
    circle: gee::Circle<f32>,
    inner_radius_over_radius: f32,
    tips: u32,
    options: Options,
}

impl Default for RegularStarBuilder {
    fn default() -> Self {
        let circle = Default::default();
        Self {
            circle,
            inner_radius_over_radius: 0.5,
            tips: 5,
            options: Default::default(),
        }
    }
}

impl RegularStarBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn pentagram() -> Self {
        Self::default().with_tips(5)
    }

    pub fn hexagram() -> Self {
        Self::default().with_tips(6)
    }

    pub fn with_tips(mut self, tips: u32) -> Self {
        assert!(tips >= 3, "can't build a star with `{}` tips", tips);
        self.tips = tips;
        self
    }

    pub fn with_center(mut self, center: gee::Point<f32>) -> Self {
        self.circle.center = center;
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.circle.radius = radius;
        self
    }

    pub fn with_inner_radius_ratio(mut self, inner_radius_over_radius: f32) -> Self {
        self.inner_radius_over_radius = inner_radius_over_radius;
        self
    }

    options_forwarder! {}

    fn points(&self) -> impl Iterator<Item = tess::math::Point> + Clone {
        let top_angle = gee::Angle::FRAC_PI_2();
        let inner_offset = gee::Angle::PI() / self.tips as f32;
        let inner_circle = {
            let mut inner_circle = self.circle;
            inner_circle.radius *= self.inner_radius_over_radius;
            inner_circle
        };
        self.circle
            .circle_points(self.tips, top_angle)
            .interleave(inner_circle.circle_points(self.tips, top_angle + inner_offset))
            .map(|p| crate::point(p))
    }

    pub fn try_build(self) -> Result<Poly, tess::TessellationError> {
        crate::try_build(self)
    }

    pub fn build(self) -> Poly {
        crate::build(self)
    }
}

impl PolyBuilder for RegularStarBuilder {
    fn build_in_place(
        self,
        vertex_buffers: &mut tess::VertexBuffers<Vertex, u32>,
    ) -> Result<(), tess::TessellationError> {
        self.options.texture_aspect_ratio;
        let _count = match self.options.tessellation {
            Tessellation::Fill => tess::basic_shapes::fill_convex_polyline(
                self.points(),
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
            Tessellation::Stroke => tess::basic_shapes::stroke_polyline(
                self.points(),
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
