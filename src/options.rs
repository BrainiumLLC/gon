use crate::tess;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct StrokeOptions {
    pub texture_aspect_ratio: f32,
    pub stroke_width: f32,
}

impl Default for StrokeOptions {
    fn default() -> Self {
        Self {
            texture_aspect_ratio: 1.0,
            stroke_width: 1.0,
        }
    }
}

impl StrokeOptions {
    pub fn new(stroke_width: f32) -> Self {
        Self::default().with_stroke_width(stroke_width)
    }

    pub fn with_stroke_width(mut self, stroke_width: f32) -> Self {
        self.stroke_width = stroke_width;
        self
    }

    /// This controls the rate at which the `y` texture coordinate reaches 1.
    ///
    /// Note: `Poly`'s generated with a stroke assume the texture will use a tiling Sampler.
    pub fn with_texture_aspect_ratio(mut self, texture_aspect_ratio: f32) -> Self {
        self.texture_aspect_ratio = texture_aspect_ratio;
        self
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Options {
    pub stroke_options: Option<StrokeOptions>,
    pub tolerance: f32,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            stroke_options: None,
            tolerance: tess::FillOptions::DEFAULT_TOLERANCE,
        }
    }
}

impl Options {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_fill(mut self) -> Self {
        self.stroke_options = None;
        self
    }

    pub fn with_stroke(mut self, stroke_width: f32) -> Self {
        self.stroke_options = self
            .stroke_options
            .map(|stroke_options| stroke_options.with_stroke_width(stroke_width))
            .or_else(|| Some(StrokeOptions::new(stroke_width)));
        self
    }

    pub fn with_stroke_opts(mut self, stroke_options: StrokeOptions) -> Self {
        self.stroke_options = Some(stroke_options);
        self
    }

    pub fn with_tolerance(mut self, tolerance: f32) -> Self {
        self.tolerance = tolerance;
        self
    }

    pub(crate) fn fill_options(&self) -> tess::FillOptions {
        assert!(self.stroke_options.is_none());
        tess::FillOptions::default().with_tolerance(self.tolerance)
    }

    pub(crate) fn stroke_options(&self) -> tess::StrokeOptions {
        let StrokeOptions {
            stroke_width,
            texture_aspect_ratio: _,
        } = self.stroke_options.clone().unwrap();
        tess::StrokeOptions::default()
            .with_tolerance(self.tolerance)
            .with_line_width(stroke_width)
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

macro_rules! stroke {
    (public) => {
        _options_forwarder! {
            with_stroke(stroke_width: f32),
            with_stroke_opts(stroke_options: StrokeOptions),
            with_tolerance(tolerance: f32),
        }
    };

    (private) => {
        fn _with_stroke(mut self, stroke_width: f32) -> Self {
            self.options = self.options.with_stroke(stroke_width);
            self
        }

        fn _with_stroke_opts(mut self, stroke_options: StrokeOptions) -> Self {
            self.options = self.options.with_stroke_opts(stroke_options);
            self
        }

        _options_forwarder! {
            with_tolerance(tolerance: f32),
        }
    };
}

macro_rules! fill {
    () => {
        pub fn with_fill(mut self) -> Self {
            self.options = self.options.with_fill();
            self
        }
    };
}
