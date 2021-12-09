use crate::{
    options::{Options, StrokeOptions},
    tess, PolyBuilder,
};
use gee::{Point, Rect, Size};

pub const DEFAULT_RADIUS: f32 = 30.0;

#[derive(Clone, Debug)]
pub struct RoundRectBuilder {
    rect: Rect,
    top_left_radius: f32,
    top_right_radius: f32,
    bottom_right_radius: f32,
    bottom_left_radius: f32,
    options: Options,
}

impl Default for RoundRectBuilder {
    fn default() -> Self {
        Self {
            rect: Rect::from_top_left(Point::zero(), Size::square(DEFAULT_RADIUS * 3.0)),
            top_left_radius: DEFAULT_RADIUS,
            top_right_radius: DEFAULT_RADIUS,
            bottom_right_radius: DEFAULT_RADIUS,
            bottom_left_radius: DEFAULT_RADIUS,
            options: Default::default(),
        }
    }
}

impl RoundRectBuilder {
    pub fn new(rect: Rect) -> Self {
        Self::default().with_rect(rect)
    }

    pub fn with_rect(mut self, rect: Rect) -> Self {
        self.rect = rect;
        self
    }

    pub fn with_clockwise_radii(
        self,
        top_left: f32,
        top_right: f32,
        bottom_right: f32,
        bottom_left: f32,
    ) -> Self {
        self.with_top_left_radius(top_left)
            .with_top_right_radius(top_right)
            .with_bottom_right_radius(bottom_right)
            .with_bottom_left_radius(bottom_left)
    }

    pub fn with_uniform_radius(self, radius: f32) -> Self {
        self.with_clockwise_radii(radius, radius, radius, radius)
    }

    pub fn with_top_radius(self, radius: f32) -> Self {
        self.with_top_left_radius(radius)
            .with_top_right_radius(radius)
    }

    pub fn with_left_radius(self, radius: f32) -> Self {
        self.with_top_left_radius(radius)
            .with_bottom_left_radius(radius)
    }

    pub fn with_right_radius(self, radius: f32) -> Self {
        self.with_top_right_radius(radius)
            .with_bottom_right_radius(radius)
    }

    pub fn with_bottom_radius(self, radius: f32) -> Self {
        self.with_bottom_right_radius(radius)
            .with_bottom_left_radius(radius)
    }

    pub fn with_top_left_radius(mut self, radius: f32) -> Self {
        self.top_left_radius = radius;
        self
    }

    pub fn with_top_right_radius(mut self, radius: f32) -> Self {
        self.top_right_radius = radius;
        self
    }

    pub fn with_bottom_right_radius(mut self, radius: f32) -> Self {
        self.bottom_right_radius = radius;
        self
    }

    pub fn with_bottom_left_radius(mut self, radius: f32) -> Self {
        self.bottom_left_radius = radius;
        self
    }

    stroke!(public);

    fill!();

    build!();
}

impl PolyBuilder for RoundRectBuilder {
    fn options(&self) -> &Options {
        &self.options
    }

    fn bounding_rect(&self) -> Rect {
        self.rect
    }

    fn build<B: tess::path::traits::PathBuilder>(self, builder: &mut B) {
        builder.add_rounded_rectangle(
            &self.rect.into(),
            &tess::path::builder::BorderRadii {
                top_left: self.top_left_radius,
                top_right: self.top_right_radius,
                bottom_right: self.bottom_right_radius,
                bottom_left: self.bottom_left_radius,
            },
            tess::path::Winding::Negative,
        );
    }
}
