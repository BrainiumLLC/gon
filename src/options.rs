use crate::tess;

#[derive(Clone, Debug)]
pub struct Options {
    pub color: [f32; 4],
    pub tessellation: Tessellation,
    pub texture_aspect_ratio: f32,
    pub stroke_width: f32,
    pub tolerance: f32,
    _prevent_destructuring: (),
}

impl Default for Options {
    fn default() -> Self {
        Self {
            color: [1.0; 4],
            tessellation: Default::default(),
            texture_aspect_ratio: 1.0,
            stroke_width: 1.0,
            tolerance: tess::FillOptions::DEFAULT_TOLERANCE,
            _prevent_destructuring: (),
        }
    }
}

impl Options {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_fill(mut self) -> Self {
        self.tessellation = Tessellation::Fill;
        self
    }

    pub fn with_stroke(mut self) -> Self {
        self.tessellation = Tessellation::Stroke;
        self
    }

    pub fn with_color(mut self, color: [f32; 4]) -> Self {
        self.color = color;
        self
    }

    pub fn with_stroke_width(mut self, stroke_width: f32) -> Self {
        self.stroke_width = stroke_width;
        self
    }

    pub fn with_texture_aspect_ratio(mut self, texture_aspect_ratio: f32) -> Self {
        self.texture_aspect_ratio = texture_aspect_ratio;
        self
    }

    pub fn with_tolerance(mut self, tolerance: f32) -> Self {
        self.tolerance = tolerance;
        self
    }

    pub(crate) fn fill_options(&self) -> tess::FillOptions {
        assert_eq!(self.tessellation, Tessellation::Fill);
        tess::FillOptions::default()
            .with_normals(false)
            .with_tolerance(self.tolerance)
            .assume_no_intersections()
    }

    pub(crate) fn stroke_options(&self) -> tess::StrokeOptions {
        assert_eq!(self.tessellation, Tessellation::Stroke);
        tess::StrokeOptions::default()
            .with_tolerance(self.tolerance)
            .with_line_width(self.stroke_width)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tessellation {
    Fill,
    Stroke,
}

impl Default for Tessellation {
    fn default() -> Self {
        Tessellation::Fill
    }
}

macro_rules! _options_forwarder {
    ($(#[$attr:meta])*$name:ident($value:ident: $t:ty)) => {
        $(#[$attr])*
        pub fn $name(mut self, $value: $t) -> Self {
            self.options = self.options.$name($value);
            self
        }
    };
    ($($name:ident($value:ident: $t:ty)),* $(,)*) => {
        $(_options_forwarder!{$name($value:$t)})*
    };
}

macro_rules! options_forwarder {
    (fixed_tessellation) => {
        _options_forwarder!{
            with_color(color: [f32; 4]),
            with_stroke_width(stroke_width: f32),
            with_texture_aspect_ratio(texture_aspect_ratio: f32),
            with_tolerance(tolerance: f32),
        }
    };
    () => {
        options_forwarder!{fixed_tessellation}

        pub fn with_fill(mut self) -> Self {
            self.options = self.options.with_fill();
            self
        }

        pub fn with_stroke(mut self) -> Self {
            self.options = self.options.with_stroke();
            self
        }
    };
}
