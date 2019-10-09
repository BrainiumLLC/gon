use crate::{
    options::{Options, StrokeOptions},
    tess,
    vertex::{FillVertexConstructor, StrokeVertexConstructor, Vertex},
    Poly, PolyBuilder, DEFAULT_RADIUS, DEFAULT_START_ANGLE,
};

#[derive(Clone, Debug)]
pub struct RegularPolyBuilder {
    circle: gee::Circle<f32>,
    sides: u32,
    start_angle: gee::Angle<f32>,
    options: Options,
}

impl Default for RegularPolyBuilder {
    fn default() -> Self {
        Self {
            circle: gee::Circle {
                radius: DEFAULT_RADIUS,
                ..Default::default()
            },
            sides: 3,
            start_angle: DEFAULT_START_ANGLE,
            options: Default::default(),
        }
    }
}

impl RegularPolyBuilder {
    pub fn new(sides: u32) -> Self {
        assert!(
            sides >= 3,
            "`Poly`'s must have atleast 3 sides, but this one has {}",
            sides
        );
        let mut result = Self::default();
        result.sides = sides;
        result
    }

    pub fn triangle() -> Self {
        Self::new(3)
    }

    pub fn quadrilateral() -> Self {
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

    pub fn with_center(mut self, center: gee::Point<f32>) -> Self {
        self.circle.center = center;
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.circle.radius = radius;
        self
    }

    pub fn with_rotation(mut self, start_angle: gee::Angle<f32>) -> Self {
        self.start_angle = start_angle;
        self
    }

    options_forwarder! {}

    fn points(&self) -> impl Iterator<Item = tess::math::Point> + Clone {
        self.circle
            .circle_points(self.sides, self.start_angle)
            .map(|point| tess::math::point(point.x, point.y))
    }

    pub fn try_build(self) -> Result<Poly, tess::TessellationError> {
        crate::try_build(self)
    }

    pub fn build(self) -> Poly {
        crate::build(self)
    }
}

impl PolyBuilder for RegularPolyBuilder {
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
