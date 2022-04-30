use std::{fmt, str::FromStr};

use crate::Color;
use glib::translate::*;

impl Color {
    pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Color {
        skip_assert_initialized!();
        unsafe {
            Self::unsafe_from(ffi::WebKitColor {
                red,
                green,
                blue,
                alpha,
            })
        }
    }

    pub fn red(&self) -> f64 {
        self.inner.red
    }

    pub fn set_red(&mut self, red: f64) {
        self.inner.red = red;
    }

    pub fn green(&self) -> f64 {
        self.inner.green
    }

    pub fn set_green(&mut self, green: f64) {
        self.inner.green = green;
    }

    pub fn blue(&self) -> f64 {
        self.inner.blue
    }

    pub fn set_blue(&mut self, blue: f64) {
        self.inner.blue = blue;
    }

    pub fn alpha(&self) -> f64 {
        self.inner.alpha
    }

    pub fn set_alpha(&mut self, alpha: f64) {
        self.inner.alpha = alpha;
    }

    /// Creates a new [`Color`] using the string representation.
    ///
    /// This function only allows use of [standard html color names] or a hex value.
    ///
    /// ```
    /// use wpewebkit::Color;
    ///
    /// let indigo = Color::parse("Indigo").unwrap();
    /// let indigo_hex = Color::parse("#4B0082").unwrap();
    ///
    /// assert_eq!(indigo.red(), indigo_hex.red());
    /// assert_eq!(indigo.green(), indigo_hex.green());
    /// assert_eq!(indigo.blue(), indigo_hex.blue());
    /// assert_eq!(indigo.alpha(), indigo_hex.alpha());
    /// ```
    ///
    /// [standard html color names]: https://htmlcolorcodes.com/color-names
    #[doc(alias = "webkit_color_parse")]
    pub fn parse(color_str: &str) -> Result<Color, glib::BoolError> {
        skip_assert_initialized!();
        unsafe {
            let mut color = Color::uninitialized();
            glib::result_from_gboolean!(
                ffi::webkit_color_parse(color.to_glib_none_mut().0, color_str.to_glib_none().0),
                "Can't parse color"
            )
            .map(|_| color)
        }
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Color")
            .field("red", &self.red())
            .field("green", &self.green())
            .field("blue", &self.blue())
            .field("alpha", &self.alpha())
            .finish()
    }
}

impl FromStr for Color {
    type Err = glib::BoolError;

    /// Same as [`Color::parse`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        skip_assert_initialized!();
        Self::parse(s)
    }
}
