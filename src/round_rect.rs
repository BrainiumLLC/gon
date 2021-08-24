use crate::{
    options::{Options, StrokeOptions},
    tess, PolyBuilder,
};
use gee::{Point, Rect, Size};

pub const DEFAULT_RADIUS: f32 = 30.0;

#[derive(Clone, Debug)]
pub struct RoundRectBuilder {
    rect: Rect,
    radius: f32,
    options: Options,
}

impl Default for RoundRectBuilder {
    fn default() -> Self {
        Self {
            rect: Rect::from_top_left(Point::zero(), Size::square(DEFAULT_RADIUS * 3.0)),
            radius: DEFAULT_RADIUS,
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

    pub fn with_corner_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
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
            &tess::path::builder::BorderRadii::new(self.radius),
            tess::path::Winding::Negative,
        );
    }
}
