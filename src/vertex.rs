use crate::{tess, StrokeOptions};
use gee::{Point, Rect, Vector};

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub pos: Point,
    pub tex_coord: Point,
}

pub(crate) struct FillVertexConstructor {
    // cached version of gee::Rect in the format we want
    top_left: Point,
    scale: Vector,
}

impl FillVertexConstructor {
    pub fn new(bounding_box: Rect) -> Self {
        Self {
            top_left: bounding_box.top_left(),
            scale: bounding_box.size().to_vector().map(f32::recip),
        }
    }
}

impl tess::FillVertexConstructor<Vertex> for FillVertexConstructor {
    fn new_vertex(&mut self, vertex: tess::FillVertex) -> Vertex {
        let pos = Point::from(vertex.position());
        let tex_coord = (pos - self.top_left).to_point() * self.scale;
        Vertex { pos, tex_coord }
    }
}

pub(crate) struct StrokeVertexConstructor {
    options: StrokeOptions,
}

impl StrokeVertexConstructor {
    pub fn new(options: StrokeOptions) -> Self {
        Self { options }
    }
}

impl tess::StrokeVertexConstructor<Vertex> for StrokeVertexConstructor {
    fn new_vertex(&mut self, vertex: tess::StrokeVertex) -> Vertex {
        Vertex {
            pos: vertex.position().into(),
            tex_coord: Point::new(
                match vertex.side() {
                    tess::Side::Left => 1.0,
                    tess::Side::Right => 0.0,
                },
                vertex.advancement() / self.options.stroke_width
                    * self.options.texture_aspect_ratio,
            ),
        }
    }
}
