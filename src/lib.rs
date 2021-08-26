#[macro_use]
mod options;

macro_rules! build {
    () => {
        pub fn try_build(self) -> Result<$crate::Poly, $crate::Error> {
            $crate::Poly::try_from_builder(self)
        }

        pub fn build(self) -> $crate::Poly {
            $crate::Poly::from_builder(self)
        }
    };
}

mod bezier;
mod circle;
mod free_poly;
mod line_segment;
mod regular_poly;
mod round_rect;
mod star;
mod vertex;

pub use self::{
    bezier::{BezierBuilder, BezierSegment, ControlPoint},
    circle::CircleBuilder,
    free_poly::FreePolyBuilder,
    line_segment::LineSegmentBuilder,
    options::StrokeOptions,
    regular_poly::RegularPolyBuilder,
    round_rect::RoundRectBuilder,
    star::StarBuilder,
    vertex::Vertex,
};
use self::{
    options::Options,
    vertex::{FillVertexConstructor, StrokeVertexConstructor},
};
use gee::{Angle, Direction, Rect};
use lyon_tessellation as tess;
use tess::path::traits::Build as _;
use thiserror::Error;

pub const DEFAULT_RADIUS: f32 = 50.0;

pub fn default_start_angle() -> Angle {
    Direction::North.angle()
}

// TODO: lyon's error type doesn't impl `Display`/`Error`
#[derive(Debug, Error)]
#[error("Tesselation failed: {0:?}")]
pub struct Error(tess::TessellationError);

impl From<tess::TessellationError> for Error {
    fn from(err: tess::TessellationError) -> Self {
        Self(err)
    }
}

/// Tesselated polygon vertices.
#[derive(Clone, Debug)]
pub struct Poly {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Poly {
    fn try_from_builder<T: PolyBuilder>(poly: T) -> Result<Self, Error> {
        let mut buf = tess::VertexBuffers::new();
        let options = poly.options();
        let _count = match options
            .stroke_options
            .clone()
            .map(StrokeVertexConstructor::new)
        {
            None => {
                let mut tessellator = tess::FillTessellator::new();
                let fill_options = options.fill_options();
                let mut buf_builder = tess::BuffersBuilder::new(
                    &mut buf,
                    FillVertexConstructor::new(poly.bounding_rect()),
                );
                let mut builder = tessellator.builder(&fill_options, &mut buf_builder);
                poly.build(&mut builder);
                builder.build()?;
            }
            Some(vertex_constructor) => {
                let mut tessellator = tess::StrokeTessellator::new();
                let stroke_options = options.stroke_options();
                let mut buf_builder = tess::BuffersBuilder::new(&mut buf, vertex_constructor);
                let mut builder = tessellator.builder(&stroke_options, &mut buf_builder);
                poly.build(&mut builder);
                builder.build()?;
            }
        };
        Ok(Self {
            vertices: buf.vertices,
            indices: buf.indices,
        })
    }

    fn from_builder<T: PolyBuilder>(builder: T) -> Poly {
        Self::try_from_builder(builder).expect("failed to build `Poly`")
    }
}

trait PolyBuilder {
    fn options(&self) -> &Options;

    fn bounding_rect(&self) -> Rect;

    fn build<B: tess::path::traits::PathBuilder>(self, builder: &mut B);
}
