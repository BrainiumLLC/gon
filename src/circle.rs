use crate::{
    options::{Options, StrokeOptions},
    tess, PolyBuilder,
};
use gee::{Circle, Rect};

#[derive(Clone, Debug, Default)]
pub struct CircleBuilder {
    pub circle: Circle,
    pub options: Options,
}

impl CircleBuilder {
    pub fn new(circle: Circle) -> Self {
        Self::default().with_circle(circle)
    }

    pub fn with_circle(mut self, circle: Circle) -> Self {
        self.circle = circle;
        self
    }

    stroke!(public);

    fill!();

    build!();
}

impl PolyBuilder for CircleBuilder {
    fn options(&self) -> &Options {
        &self.options
    }

    fn bounding_rect(&self) -> Rect {
        self.circle.bounding_rect()
    }

    fn build<B: tess::path::traits::PathBuilder>(self, builder: &mut B) {
        builder.add_circle(
            self.circle.center().into(),
            self.circle.radius(),
            tess::path::Winding::Positive,
        );
    }
}
