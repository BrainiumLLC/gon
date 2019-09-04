#[macro_use]
mod options;

mod circle;
mod line_segment;
mod regular_star;
mod regular_poly;
mod vertex;

pub use circle::CircleBuilder;
pub use line_segment::LineSegmentBuilder;
pub use lyon_tessellation as tess;
pub use regular_poly::RegularPolyBuilder;
pub use regular_star::RegularStarBuilder;
pub use vertex::Vertex;

#[derive(Clone, Debug)]
pub struct Poly {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

trait PolyBuilder {
    fn build_in_place(
        self,
        vertex_buffers: &mut tess::VertexBuffers<Vertex, u32>,
    ) -> Result<(), tess::TessellationError>;
}

fn try_build<T: PolyBuilder>(builder: T) -> Result<Poly, tess::TessellationError> {
    let mut output: tess::VertexBuffers<Vertex, u32> = tess::VertexBuffers::new();
    builder.build_in_place(&mut output)?;
    Ok(Poly {
        vertices: output.vertices,
        indices: output.indices,
    })
}

fn build<T: PolyBuilder>(builder: T) -> Poly {
    try_build(builder).expect("failed to build `Poly`")
}

fn point(gee: gee::Point<f32>) -> tess::math::Point {
    tess::math::point(gee.x, gee.y)
}
