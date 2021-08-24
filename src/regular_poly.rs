use crate::{
    default_start_angle,
    options::{Options, StrokeOptions},
    tess, FreePolyBuilder, PolyBuilder, DEFAULT_RADIUS,
};
use gee::{Angle, Circle, Point, Rect};

#[derive(Clone, Debug)]
pub struct RegularPolyBuilder {
    circle: Circle,
    sides: u32,
    start_angle: Angle,
    options: Options,
}

impl Default for RegularPolyBuilder {
    fn default() -> Self {
        Self {
            circle: Circle::from_radius(DEFAULT_RADIUS),
            sides: 3,
            start_angle: default_start_angle(),
            options: Default::default(),
        }
    }
}

impl RegularPolyBuilder {
    pub fn new(sides: u32) -> Self {
        Self::default().with_sides(sides)
    }

    pub fn triangle() -> Self {
        Self::new(3)
    }

    pub fn square() -> Self {
        Self::new(4)
    }

    pub fn pentagon() -> Self {
        Self::new(5)
    }

    pub fn hexagon() -> Self {
        Self::new(6)
    }

    pub fn octagon() -> Self {
        Self::new(8)
    }

    pub fn decagon() -> Self {
        Self::new(10)
    }

    pub fn with_sides(mut self, sides: u32) -> Self {
        assert!(
            sides >= 3,
            "regular polygons must have at least 3 sides, but this one has {}",
            sides
        );
        self.sides = sides;
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

    stroke!(public);

    fill!();

    build!();
}

impl PolyBuilder for RegularPolyBuilder {
    fn options(&self) -> &Options {
        &self.options
    }

    fn bounding_rect(&self) -> Rect {
        self.circle.bounding_rect()
    }

    fn build<B: tess::path::traits::PathBuilder>(self, builder: &mut B) {
        PolyBuilder::build(
            FreePolyBuilder::from_parts(
                self.circle.circle_points(self.sides, self.start_angle),
                true,
                Some(self.bounding_rect()),
                self.options,
            ),
            builder,
        );
    }
}
