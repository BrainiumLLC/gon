#[macro_use]
mod options;

mod circle;
mod line_segment;
mod regular_poly;
mod round_rect;
mod star;
mod vertex;

pub use circle::CircleBuilder;
pub use gee;
pub use line_segment::LineSegmentBuilder;
pub use lyon_tessellation as tess;
pub use options::StrokeOptions;
pub use regular_poly::RegularPolyBuilder;
pub use round_rect::RoundRectBuilder;
pub use star::StarBuilder;
pub use vertex::{FillVertexConstructor, StrokeVertexConstructor, Vertex};

pub const DEFAULT_RADIUS: f32 = 50.0;

pub fn default_start_angle() -> gee::Angle<f32> {
    gee::Direction::North.angle()
}

#[derive(Clone, Debug)]
pub struct Poly {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

pub trait PolyBuilder {
    fn build_in_place(
        self,
        vertex_buffers: &mut tess::VertexBuffers<Vertex, u32>,
    ) -> Result<(), tess::TessellationError>;
}

impl<T: PolyBuilder> From<T> for Poly {
    fn from(pb: T) -> Self {
        build(pb)
    }
}

pub fn try_build<T: PolyBuilder>(builder: T) -> Result<Poly, tess::TessellationError> {
    let mut output: tess::VertexBuffers<Vertex, u32> = tess::VertexBuffers::new();
    builder.build_in_place(&mut output)?;
    Ok(Poly {
        vertices: output.vertices,
        indices: output.indices,
    })
}

pub fn build<T: PolyBuilder>(builder: T) -> Poly {
    try_build(builder).expect("failed to build `Poly`")
}

fn point(gee: gee::Point<f32>) -> tess::math::Point {
    tess::math::point(gee.x, gee.y)
}
