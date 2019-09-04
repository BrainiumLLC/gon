use crate::tess;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub pos: gee::Vec2<f32>,
    pub color: gee::Vec4<f32>,
    pub tex_coord: gee::Vec2<f32>,
}

pub(crate) struct FillVertexConstructor {
    pub color: gee::Vec4<f32>,
    // cached version of gee::Rect in the format we want
    pub top_left: gee::Point<f32>,
    pub size: gee::Size<f32>,
    pub texture_aspect_ratio: f32,
}

impl FillVertexConstructor {
    pub fn new(
        color: gee::Vec4<f32>,
        bounding_box: gee::Rect<f32>,
        texture_aspect_ratio: f32,
    ) -> Self {
        Self {
            color,
            top_left: bounding_box.top_left(),
            size: bounding_box.size(),
            texture_aspect_ratio,
        }
    }
}

impl tess::VertexConstructor<tess::FillVertex, Vertex> for FillVertexConstructor {
    fn new_vertex(&mut self, vertex: tess::FillVertex) -> Vertex {
        let pos = gee::Point::new(vertex.position.x, vertex.position.y);
        let color = self.color;
        let rel_coord = pos - self.top_left;
        let tex_coord = gee::Vec2::new(
            rel_coord.dx / self.size.width,
            rel_coord.dy / self.size.height / self.texture_aspect_ratio,
        );
        Vertex {
            pos: pos.into_vec2(),
            color,
            tex_coord,
        }
    }
}

pub(crate) struct StrokeVertexConstructor {
    pub color: gee::Vec4<f32>,
    pub stroke_width: f32,
    pub texture_aspect_ratio: f32,
}

impl StrokeVertexConstructor {
    pub fn new(color: gee::Vec4<f32>, stroke_width: f32, texture_aspect_ratio: f32) -> Self {
        Self {
            color,
            stroke_width,
            texture_aspect_ratio,
        }
    }
}

impl tess::VertexConstructor<tess::StrokeVertex, Vertex> for StrokeVertexConstructor {
    fn new_vertex(&mut self, vertex: tess::StrokeVertex) -> Vertex {
        let pos = gee::Vec2::new(vertex.position.x, vertex.position.y);
        let color = self.color;
        let x = match vertex.side {
            tess::Side::Left => 1.0,
            tess::Side::Right => 0.0,
        };
        let y = vertex.advancement / self.stroke_width * self.texture_aspect_ratio;
        let tex_coord = gee::Vec2::new(x, y);
        Vertex {
            pos,
            color,
            tex_coord,
        }
    }
}
