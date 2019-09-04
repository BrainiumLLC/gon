use crate::{
    options::{Options, Tessellation},
    tess,
    vertex::{FillVertexConstructor, StrokeVertexConstructor, Vertex},
    Poly, PolyBuilder,
};

#[derive(Clone, Debug)]
pub struct RegularNGonBuilder {
    circle: gee::Circle<f32>,
    sides: u32,
    options: Options,
}

impl Default for RegularNGonBuilder {
    fn default() -> Self {
        Self {
            circle: Default::default(),
            sides: 3,
            options: Default::default(),
        }
    }
}

impl RegularNGonBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn triangle() -> Self {
        Self::default().with_sides(3)
    }

    pub fn quadrilateral() -> Self {
        Self::default().with_sides(4)
    }

    pub fn pentagon() -> Self {
        Self::default().with_sides(5)
    }

    pub fn hexagon() -> Self {
        Self::default().with_sides(6)
    }

    pub fn octagon() -> Self {
        Self::default().with_sides(8)
    }

    pub fn decagon() -> Self {
        Self::default().with_sides(10)
    }

    pub fn with_center(mut self, center: gee::Point<f32>) -> Self {
        self.circle.center = center;
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.circle.radius = radius;
        self
    }

    pub fn with_sides(mut self, sides: u32) -> Self {
        self.sides = sides;
        self
    }

    options_forwarder! {}

    fn points(&self) -> impl Iterator<Item = tess::math::Point> + Clone {
        self.circle
            .circle_points(self.sides, gee::Angle::FRAC_PI_2())
            .map(|point| tess::math::point(point.x, point.y))
    }

    pub fn try_build(self) -> Result<Poly, tess::TessellationError> {
        crate::try_build(self)
    }

    pub fn build(self) -> Poly {
        crate::build(self)
    }
}

impl PolyBuilder for RegularNGonBuilder {
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
