use crate::ui::{
    canvas::Viewport,
    display::Color,
    geometry::{Offset, Point, Rect},
};

use super::octant::octant_points;

use crate::trezorhal::gdc;

// A reference to a bitmap with additional parameters,
// such as foreground/background color and drawing offset
pub type BitmapRef<'a> = gdc::GdcBitmapRef<'a>;

/// Text attributes, including font, foreground/background color,
/// and drawing offset
pub type TextAttr = gdc::GdcTextAttr;

pub trait RgbCanvas {
    /// Sets the active viewport valid for all subsequent drawing operations
    fn set_viewport(&mut self, vp: Viewport);

    /// Gets the current drawing viewport previously set by set_viewport()
    /// function
    fn viewport(&self) -> Viewport;

    /// Returns dimensions of the canvas in pixels
    fn size(&self) -> Offset;

    /// Draws a filled rectangle with the specified color
    fn fill_rect(&mut self, r: Rect, color: Color);

    /// Draws a bitmap of bitmap into to the rectangle
    fn draw_bitmap(&mut self, r: Rect, bitmap: &BitmapRef);

    /// Blends two bitmaps and draws them into the specified rectangle
    fn draw_blended(&mut self, r: Rect, fg: &BitmapRef, bg: &BitmapRef);

    /// Draws text to the specified rectangle
    fn draw_opaque_text(&mut self, r: Rect, text: &str, attr: &TextAttr);

    /// Blends text with a bitmap and draws it into the specified rectangle
    /// (ignores attr.bg_color attribute)
    fn draw_blended_text(&mut self, r: Rect, text: &str, attr: &TextAttr, bg: &BitmapRef);

    fn set_window_hint(&mut self, _r: Rect) {
        // Empty default implementation
    }

    /// Returns the dimensions of the canvas as a rectangle with the top-left at
    /// (0,0)
    fn bounds(&self) -> Rect {
        Rect::from_size(self.size())
    }

    fn witdh(&self) -> i16 {
        self.size().x
    }

    fn height(&self) -> i16 {
        self.size().y
    }

    fn set_window(&mut self, window: Rect) -> Viewport {
        let viewport = self.viewport();
        self.set_viewport(viewport.relative_window(window));
        viewport
    }

    fn set_clip(&mut self, clip: Rect) -> Viewport {
        let viewport = self.viewport();
        self.set_viewport(viewport.relative_clip(clip));
        viewport
    }

    // Draw a pixel at specified coordinates
    // (this default implementation is highly inefficient)
    fn draw_pixel(&mut self, pt: Point, color: Color) {
        self.fill_rect(Rect::from_top_left_and_size(pt, Offset::new(1, 1)), color);
    }

    fn draw_round_rect(&mut self, r: Rect, radius: i16, color: Color) {
        let (split, _, _, _, _) = octant_points(radius).last().unwrap();

        let b = Rect {
            y1: r.y0 + radius - split + 1,
            ..r
        };

        if self.viewport().contains(b) {
            for (x, y, _, _, last) in octant_points(radius) {
                let pt_l = Point::new(r.x0 + radius - y, r.y0 + radius - x);
                let pt_r = Point::new(r.x1 - radius + y - 1, r.y0 + radius - x);
                if x == radius && last {
                    self.fill_rect(Rect::new(pt_l, pt_r.onright().under()), color);
                } else {
                    self.draw_pixel(pt_l, color);
                    self.draw_pixel(pt_r, color);
                }
            }
        }

        let b = Rect {
            y0: r.y0 + radius - split + 1,
            y1: r.y0 + radius + 1,
            ..r
        };

        if self.viewport().contains(b) {
            for (x, y, _, _, _) in octant_points(radius) {
                let pt_l = Point::new(r.x0 + radius - x, r.y0 + radius - y);
                let pt_r = Point::new(r.x1 - radius + x - 1, r.y0 + radius - y);
                self.draw_pixel(pt_l, color);
                self.draw_pixel(pt_r, color);
            }
        }

        self.fill_rect(
            Rect {
                x0: r.x0,
                y0: r.y0 + radius + 1,
                x1: r.x0 + 1,
                y1: r.y1 - radius - 1,
            },
            color,
        );

        self.fill_rect(
            Rect {
                x0: r.x1 - 1,
                y0: r.y0 + radius + 1,
                x1: r.x1,
                y1: r.y1 - radius - 1,
            },
            color,
        );

        let b = Rect {
            y0: r.y1 - radius - 1,
            y1: r.y1 - radius - 1 + split,
            ..r
        };

        if self.viewport().contains(b) {
            for (x, y, _, _, _) in octant_points(radius) {
                let pt_l = Point::new(r.x0 + radius - x, r.y1 - radius - 1 + y);
                let pt_r = Point::new(r.x1 - radius + x - 1, r.y1 - radius - 1 + y);
                self.draw_pixel(pt_l, color);
                self.draw_pixel(pt_r, color);
            }
        }

        let b = Rect {
            y0: r.y1 - radius - 1 + split,
            ..r
        };

        if self.viewport().contains(b) {
            for (x, y, _, _, last) in octant_points(radius) {
                let pt_l = Point::new(r.x0 + radius - y, r.y1 - radius - 1 + x);
                let pt_r = Point::new(r.x1 - radius + y - 1, r.y1 - radius - 1 + x);

                if x == radius && last {
                    self.fill_rect(Rect::new(pt_l, pt_r.onright().under()), color);
                } else {
                    self.draw_pixel(pt_l, color);
                    self.draw_pixel(pt_r, color);
                }
            }
        }
    }

    fn fill_round_rect(&mut self, r: Rect, radius: i16, color: Color) {
        let (split, _, _, _, _) = octant_points(radius).last().unwrap();

        let b = Rect {
            y1: r.y0 + radius - split + 1,
            ..r
        };

        if self.viewport().contains(b) {
            for (x, y, _, _, last) in octant_points(radius) {
                if last {
                    let pt_l = Point::new(r.x0 + radius - y, r.y0 + radius - x);
                    let pt_r = Point::new(r.x1 - radius + y - 1, r.y0 + radius - x);
                    self.fill_rect(Rect::new(pt_l, pt_r.onright().under()), color);
                }
            }
        }

        let b = Rect {
            y0: r.y0 + radius - split + 1,
            y1: r.y0 + radius + 1,
            ..r
        };

        if self.viewport().contains(b) {
            for (x, y, _, _, _) in octant_points(radius) {
                let pt_l = Point::new(r.x0 + radius - x, r.y0 + radius - y);
                let pt_r = Point::new(r.x1 - radius + x - 1, r.y0 + radius - y);
                self.fill_rect(Rect::new(pt_l, pt_r.onright().under()), color);
            }
        }

        self.fill_rect(
            Rect {
                x0: r.x0,
                y0: r.y0 + radius + 1,
                x1: r.x1,
                y1: r.y1 - radius - 1,
            },
            color,
        );

        let b = Rect {
            y0: r.y1 - radius - 1,
            y1: r.y1 - radius - 1 + split,
            ..r
        };

        if self.viewport().contains(b) {
            for (x, y, _, _, _) in octant_points(radius) {
                let pt_l = Point::new(r.x0 + radius - x, r.y1 - radius - 1 + y);
                let pt_r = Point::new(r.x1 - radius + x - 1, r.y1 - radius - 1 + y);
                self.fill_rect(Rect::new(pt_l, pt_r.onright().under()), color);
            }
        }

        let b = Rect {
            y0: r.y1 - radius - 1 + split,
            ..r
        };

        if self.viewport().contains(b) {
            for (x, y, _, _, last) in octant_points(radius) {
                if last {
                    let pt_l = Point::new(r.x0 + radius - y, r.y1 - radius - 1 + x);
                    let pt_r = Point::new(r.x1 - radius + y - 1, r.y1 - radius - 1 + x);
                    self.fill_rect(Rect::new(pt_l, pt_r.onright().under()), color);
                }
            }
        }
    }

    // Draws filled circle with the specified center and the radius
    fn fill_circle(&mut self, center: Point, radius: i16, color: Color) {
        let (split, _, _, _, _) = octant_points(radius).last().unwrap();

        let r = Rect::new(
            Point::new(center.x - radius, center.y - radius),
            Point::new(center.x + radius + 1, center.y - split + 1),
        );

        if self.viewport().contains(r) {
            for (x, y, _, _, last) in octant_points(radius) {
                if last {
                    let pt_l = Point::new(center.x - y, center.y - x);
                    let pt_r = Point::new(center.x + y, center.y - x);
                    self.fill_rect(Rect::new(pt_l, pt_r.onright().under()), color);
                }
            }
        }

        let r = Rect::new(
            Point::new(center.x - radius, center.y - split),
            Point::new(center.x + radius + 1, center.y + 1),
        );

        if self.viewport().contains(r) {
            for (x, y, _, _, _) in octant_points(radius) {
                let pt_l = Point::new(center.x - x, center.y - y);
                let pt_r = Point::new(center.x + x, center.y - y);
                self.fill_rect(Rect::new(pt_l, pt_r.onright().under()), color);
            }
        }

        let r = Rect::new(
            Point::new(center.x - radius, center.y),
            Point::new(center.x + radius + 1, center.y + split + 1),
        );

        if self.viewport().contains(r) {
            for (x, y, _, _, _) in octant_points(radius) {
                let pt_l = Point::new(center.x - x, center.y + y);
                let pt_r = Point::new(center.x + x, center.y + y);
                self.fill_rect(Rect::new(pt_l, pt_r.onright().under()), color);
            }
        }

        let r = Rect::new(
            Point::new(center.x - radius, center.y + split),
            Point::new(center.x + radius + 1, center.y + radius + 1),
        );

        if self.viewport().contains(r) {
            for (x, y, _, _, last) in octant_points(radius) {
                if last {
                    let pt_l = Point::new(center.x - y, center.y + x);
                    let pt_r = Point::new(center.x + y, center.y + x);
                    self.fill_rect(Rect::new(pt_l, pt_r.onright().under()), color);
                }
            }
        }
    }

    // Draws circle with the specified center and the radius
    fn draw_circle(&mut self, center: Point, radius: i16, color: Color) {
        let (split, _, _, _, _) = octant_points(radius).last().unwrap();

        let r = Rect::new(
            Point::new(center.x - radius, center.y - radius),
            Point::new(center.x + radius + 1, center.y - split + 1),
        );

        if self.viewport().contains(r) {
            for (x, y, _, _, _) in octant_points(radius) {
                let pt_l = Point::new(center.x - y, center.y - x);
                let pt_r = Point::new(center.x + y, center.y - x);
                self.draw_pixel(pt_l, color);
                self.draw_pixel(pt_r, color);
            }
        }

        let r = Rect::new(
            Point::new(center.x - radius, center.y - split),
            Point::new(center.x + radius + 1, center.y + 1),
        );

        if self.viewport().contains(r) {
            for (x, y, _, _, _) in octant_points(radius) {
                let pt_l = Point::new(center.x - x, center.y - y);
                let pt_r = Point::new(center.x + x, center.y - y);
                self.draw_pixel(pt_l, color);
                self.draw_pixel(pt_r, color);
            }
        }

        let r = Rect::new(
            Point::new(center.x - radius, center.y),
            Point::new(center.x + radius + 1, center.y + split + 1),
        );

        if self.viewport().contains(r) {
            for (x, y, _, _, _) in octant_points(radius) {
                let pt_l = Point::new(center.x - x, center.y + y);
                let pt_r = Point::new(center.x + x, center.y + y);
                self.draw_pixel(pt_l, color);
                self.draw_pixel(pt_r, color);
            }
        }

        let r = Rect::new(
            Point::new(center.x - radius, center.y + split),
            Point::new(center.x + radius + 1, center.y + radius + 1),
        );

        if self.viewport().contains(r) {
            for (x, y, _, _, _) in octant_points(radius) {
                let pt_l = Point::new(center.x - y, center.y + x);
                let pt_r = Point::new(center.x + y, center.y + x);
                self.draw_pixel(pt_l, color);
                self.draw_pixel(pt_r, color);
            }
        }
    }

    /// Fills the canvas background with the specified color
    fn fill_background(&mut self, color: Color) {
        self.fill_rect(self.viewport().clip, color);
    }
}

pub trait RgbCanvasEx: RgbCanvas {
    /// Returns a non-mutable reference to the underlying bitmap
    fn bitmap<'a>(&'a self) -> BitmapRef<'a>;

    /// Draws a single pixel and blends its color with the background
    /// If alpha == 255, the (foreground) pixel color is used
    /// If 0 < alpha << 255, pixel and backround colors are blended
    /// If alpha == 0, the background color is used
    fn blend_pixel(&mut self, pt: Point, color: Color, alpha: u8);

    /// Draws text (with a transparent backround) to the specified rectangle
    fn draw_text(&mut self, r: Rect, text: &str, attr: &TextAttr) {
        let offset = r.top_left() + self.viewport().origin;
        self.draw_blended_text(r, text, attr, &self.bitmap().with_offset(offset.into()));
    }

    // Blends a bitmap with the canvas background
    // (TODO: Explain better)
    fn blend_bitmap(&mut self, r: Rect, bitmap: &BitmapRef) {
        let offset = r.top_left() + self.viewport().origin;
        self.draw_blended(r, bitmap, &self.bitmap().with_offset(offset.into()));
    }

    fn blur_rect(&mut self, r: Rect, radius: usize);

    // Draws antialiased filled circle with the specified center and the radius
    fn fill_circle_aa(&mut self, center: Point, radius: i16, color: Color) {
        let (split, _, _, _, _) = octant_points(radius).last().unwrap();

        let r = Rect::new(
            Point::new(center.x - radius, center.y - radius),
            Point::new(center.x + radius + 1, center.y - split + 1),
        );

        if self.viewport().contains(r) {
            for (x, y, alpha, first, _) in octant_points(radius) {
                let pt_l = Point::new(center.x - y, center.y - x);
                let pt_r = Point::new(center.x + y, center.y - x);
                self.blend_pixel(pt_l, color, alpha);
                self.blend_pixel(pt_r, color, alpha);

                if first {
                    let r = Rect::new(pt_l.onright(), pt_r.under());
                    self.fill_rect(r, color);
                }
            }
        }

        let r = Rect::new(
            Point::new(center.x - radius, center.y - split),
            Point::new(center.x + radius + 1, center.y + 1),
        );

        if self.viewport().contains(r) {
            for (x, y, alpha, _, _) in octant_points(radius) {
                let pt_l = Point::new(center.x - x, center.y - y);
                let pt_r = Point::new(center.x + x, center.y - y);
                self.blend_pixel(pt_l, color, alpha);
                self.blend_pixel(pt_r, color, alpha);

                let r = Rect::new(pt_l.onright(), pt_r.under());
                self.fill_rect(r, color);
            }
        }

        let r = Rect::new(
            Point::new(center.x - radius, center.y),
            Point::new(center.x + radius + 1, center.y + split + 1),
        );

        if self.viewport().contains(r) {
            for (x, y, alpha, _, _) in octant_points(radius) {
                let pt_l = Point::new(center.x - x, center.y + y);
                let pt_r = Point::new(center.x + x, center.y + y);
                self.blend_pixel(pt_l, color, alpha);
                self.blend_pixel(pt_r, color, alpha);

                let r = Rect::new(pt_l.onright(), pt_r.under());
                self.fill_rect(r, color);
            }
        }

        let r = Rect::new(
            Point::new(center.x - radius, center.y + split),
            Point::new(center.x + radius + 1, center.y + radius + 1),
        );

        if self.viewport().contains(r) {
            for (x, y, alpha, first, _) in octant_points(radius) {
                let pt_l = Point::new(center.x - y, center.y + x);
                let pt_r = Point::new(center.x + y, center.y + x);
                self.blend_pixel(pt_l, color, alpha);
                self.blend_pixel(pt_r, color, alpha);

                if first {
                    let r = Rect::new(pt_l.onright(), pt_r.under());
                    self.fill_rect(r, color);
                }
            }
        }
    }

    // Draws antialiased circle with the specified center and the radius
    fn draw_circle_aa(&mut self, center: Point, radius: i16, color: Color) {
        let (split, _, _, _, _) = octant_points(radius).last().unwrap();

        let r = Rect::new(
            Point::new(center.x - radius, center.y - radius),
            Point::new(center.x + radius + 1, center.y - split + 1),
        );

        if self.viewport().contains(r) {
            for (x, y, alpha, _, _) in octant_points(radius) {
                let pt_l = Point::new(center.x - y, center.y - x);
                self.blend_pixel(pt_l, color, alpha);
                self.blend_pixel(pt_l.under(), color, 255 - alpha);
                let pt_r = Point::new(center.x + y, center.y - x);
                self.blend_pixel(pt_r, color, alpha);
                self.blend_pixel(pt_r.under(), color, 255 - alpha);
            }
        }

        let r = Rect::new(
            Point::new(center.x - radius, center.y - split),
            Point::new(center.x + radius + 1, center.y + 1),
        );

        if self.viewport().contains(r) {
            for (x, y, alpha, _, _) in octant_points(radius) {
                let pt_l = Point::new(center.x - x, center.y - y);
                self.blend_pixel(pt_l, color, alpha);
                self.blend_pixel(pt_l.onright(), color, 255 - alpha);
                let pt_r = Point::new(center.x + x, center.y - y);
                self.blend_pixel(pt_r, color, alpha);
                self.blend_pixel(pt_r.onleft(), color, 255 - alpha);
            }
        }

        let r = Rect::new(
            Point::new(center.x - radius, center.y),
            Point::new(center.x + radius + 1, center.y + split + 1),
        );

        if self.viewport().contains(r) {
            for (x, y, alpha, _, _) in octant_points(radius) {
                let pt_l = Point::new(center.x - x, center.y + y);
                self.blend_pixel(pt_l, color, alpha);
                self.blend_pixel(pt_l.onright(), color, 255 - alpha);
                let pt_r = Point::new(center.x + x, center.y + y);
                self.blend_pixel(pt_r, color, alpha);
                self.blend_pixel(pt_r.onleft(), color, 255 - alpha);
            }
        }

        let r = Rect::new(
            Point::new(center.x - radius, center.y + split),
            Point::new(center.x + radius + 1, center.y + radius + 1),
        );

        if self.viewport().contains(r) {
            for (x, y, alpha, _, _) in octant_points(radius) {
                let pt_l = Point::new(center.x - y, center.y + x);
                self.blend_pixel(pt_l, color, alpha);
                self.blend_pixel(pt_l.above(), color, 255 - alpha);
                let pt_r = Point::new(center.x + y, center.y + x);
                self.blend_pixel(pt_r, color, alpha);
                self.blend_pixel(pt_r.above(), color, 255 - alpha);
            }
        }
    }

    fn fill_round_rect_aa(&mut self, r: Rect, radius: i16, color: Color) {
        let (split, _, _, _, _) = octant_points(radius).last().unwrap();

        let b = Rect {
            y1: r.y0 + radius - split + 1,
            ..r
        };

        if self.viewport().contains(b) {
            for (x, y, alpha, first, _) in octant_points(radius) {
                let pt_l = Point::new(r.x0 + radius - y, r.y0 + radius - x);
                let pt_r = Point::new(r.x1 - radius + y - 1, r.y0 + radius - x);
                self.blend_pixel(pt_l, color, alpha);
                self.blend_pixel(pt_r, color, alpha);

                if first {
                    let inner = Rect::new(pt_l.onright(), pt_r.under());
                    self.fill_rect(inner, color);
                }
            }
        }

        let b = Rect {
            y0: r.y0 + radius - split + 1,
            y1: r.y0 + radius + 1,
            ..r
        };

        if self.viewport().contains(b) {
            for (x, y, alpha, _, _) in octant_points(radius) {
                let pt_l = Point::new(r.x0 + radius - x, r.y0 + radius - y);
                let pt_r = Point::new(r.x1 - radius + x - 1, r.y0 + radius - y);
                self.blend_pixel(pt_l, color, alpha);
                self.blend_pixel(pt_r, color, alpha);

                let inner = Rect::new(pt_l.onright(), pt_r.under());
                self.fill_rect(inner, color);
            }
        }

        self.fill_rect(
            Rect {
                x0: r.x0,
                y0: r.y0 + radius + 1,
                x1: r.x1,
                y1: r.y1 - radius - 1,
            },
            color,
        );

        let b = Rect {
            y0: r.y1 - radius - 1,
            y1: r.y1 - radius - 1 + split,
            ..r
        };

        if self.viewport().contains(b) {
            for (x, y, alpha, _, _) in octant_points(radius) {
                let pt_l = Point::new(r.x0 + radius - x, r.y1 - radius - 1 + y);
                let pt_r = Point::new(r.x1 - radius + x - 1, r.y1 - radius - 1 + y);
                self.blend_pixel(pt_l, color, alpha);
                self.blend_pixel(pt_r, color, alpha);

                let b = Rect::new(pt_l.onright(), pt_r.under());
                self.fill_rect(b, color);
            }
        }

        let b = Rect {
            y0: r.y1 - radius - 1 + split,
            ..r
        };

        if self.viewport().contains(b) {
            for (x, y, alpha, first, _) in octant_points(radius) {
                let pt_l = Point::new(r.x0 + radius - y, r.y1 - radius - 1 + x);
                self.blend_pixel(pt_l, color, alpha);
                let pt_r = Point::new(r.x1 - radius + y - 1, r.y1 - radius - 1 + x);
                self.blend_pixel(pt_r, color, alpha);

                if first {
                    let b = Rect::new(pt_l.onright(), pt_r.under());
                    self.fill_rect(b, color);
                }
            }
        }
    }
}

impl Point {
    fn onleft(self) -> Self {
        Self {
            x: self.x - 1,
            ..self
        }
    }

    fn onright(self) -> Self {
        Self {
            x: self.x + 1,
            ..self
        }
    }

    fn above(self) -> Self {
        Self {
            y: self.y - 1,
            ..self
        }
    }

    fn under(self) -> Self {
        Self {
            y: self.y + 1,
            ..self
        }
    }
}
