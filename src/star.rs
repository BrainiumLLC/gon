use crate::{
    default_start_angle,
    options::{Options, StrokeOptions},
    tess, FreePolyBuilder, PolyBuilder, DEFAULT_RADIUS,
};
use gee::{Angle, Circle, Point, Rect};
use itertools::Itertools as _;

#[derive(Clone, Debug)]
pub struct StarBuilder {
    circle: Circle,
    inner_radius_over_radius: f32,
    tips: u32,
    start_angle: Angle,
    options: Options,
}

impl Default for StarBuilder {
    fn default() -> Self {
        Self {
            circle: Circle::from_radius(DEFAULT_RADIUS),
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

    pub fn with_circle(mut self, circle: Circle) -> Self {
        self.circle = circle;
        self
    }

    pub fn with_center_and_radius(self, center: Point, radius: f32) -> Self {
        self.with_circle(Circle::new(center, radius))
    }

    pub fn with_rotation(mut self, start_angle: impl Into<Angle>) -> Self {
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

    stroke!(public);

    fill!();

    build!();
}

impl PolyBuilder for StarBuilder {
    fn options(&self) -> &Options {
        &self.options
    }

    fn bounding_rect(&self) -> Rect {
        self.circle.bounding_rect()
    }

    fn build<B: tess::path::traits::PathBuilder>(self, builder: &mut B) {
        PolyBuilder::build(
            FreePolyBuilder::from_parts(
                {
                    let top_angle = self.start_angle;
                    let inner_offset = gee::Angle::PI() / self.tips as f32;
                    let inner_circle = self.circle.scale_radius(self.inner_radius_over_radius);
                    self.circle
                        .circle_points(self.tips, top_angle)
                        .interleave(inner_circle.circle_points(self.tips, top_angle + inner_offset))
                },
                true,
                Some(self.bounding_rect()),
                self.options,
            ),
            builder,
        );
    }
}
