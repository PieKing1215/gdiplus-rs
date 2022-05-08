use std::mem::MaybeUninit;

use winapi::shared::windef::HDC;
use winapi::um::gdiplusflat::{GdipCreateFromHDC, GdipSetSmoothingMode, GdipDeleteGraphics, GdipDrawLine, GdipFillRectangle, GdipFillRectangles};
use winapi::um::gdiplusgpstubs::{GpGraphics, GpRectF};
use winapi::um::gdiplustypes::REAL;

use crate::brush::SolidBrush;
use crate::enums::SmoothingMode;
use crate::pen::Pen;
use crate::types::{Point, Result};
use crate::{panic_iferror, return_iferror};

pub struct Graphics {
    graphics: *mut GpGraphics,
}
impl Graphics {
    pub(crate) fn graphics(&self) -> *mut GpGraphics {
        self.graphics
    }

    pub fn from_hdc(hdc: HDC) -> Result<Self> {
        let mut graphics = MaybeUninit::uninit();
        return_iferror!(GdipCreateFromHDC(hdc, graphics.as_mut_ptr()));

        let graphics = unsafe { graphics.assume_init() };

        Ok(Self { graphics })
    }

    pub fn set_smoothing_mode(&mut self, smoothing_mode: SmoothingMode) -> Result<&mut Self> {
        return_iferror!(GdipSetSmoothingMode(self.graphics, smoothing_mode as u32));
        Ok(self)
    }

    pub fn with_brush<'a>(&'a mut self, brush: &'a mut SolidBrush) -> WithBrush<'a> {
        WithBrush::new(self, brush)
    }

    pub fn with_pen<'a>(&'a mut self, pen: &'a mut Pen) -> WithPen<'a> {
        WithPen::new(self, pen)
    }
}
impl Drop for Graphics {
    fn drop(&mut self) {
        panic_iferror!(GdipDeleteGraphics(self.graphics));
    }
}

pub struct WithPen<'a> {
    graphics: &'a mut Graphics,
    pen: &'a mut Pen,
    current_pos: Point,
}
impl<'a> WithPen<'a> {
    pub fn new(graphics: &'a mut Graphics, pen: &'a mut Pen) -> Self {
        Self {
            graphics,
            pen,
            current_pos: (0.0, 0.0),
        }
    }

    pub fn current_pos(&self) -> Point {
        self.current_pos
    }

    pub fn modify(&mut self, fn_: fn(&mut Pen) -> Result<()>) -> Result<&mut Self> {
        fn_(self.pen)?;
        Ok(self)
    }

    pub fn replace(&mut self, pen: &'a mut Pen) -> &mut Self {
        self.pen = pen;
        self
    }

    pub fn move_to(&mut self, point: Point) -> &mut Self {
        self.current_pos = point;
        self
    }

    pub fn line_to(&mut self, point: Point) -> Result<&mut Self> {
        return_iferror!(GdipDrawLine(
            self.graphics.graphics(),
            self.pen.pen(),
            self.current_pos.0,
            self.current_pos.1,
            point.0,
            point.1,
        ));

        self.current_pos = point;

        Ok(self)
    }

    pub fn draw_line(&mut self, from: Point, to: Point) -> Result<&mut Self> {
        return_iferror!(GdipDrawLine(
            self.graphics.graphics(),
            self.pen.pen(),
            from.0,
            from.1,
            to.0,
            to.1,
        ));

        Ok(self)
    }
}

pub struct WithBrush<'a> {
    graphics: &'a mut Graphics,
    brush: &'a mut SolidBrush,
}
impl<'a> WithBrush<'a> {
    pub fn new(graphics: &'a mut Graphics, brush: &'a mut SolidBrush) -> Self {
        Self { graphics, brush }
    }

    pub fn modify(&mut self, fn_: fn(&mut SolidBrush) -> Result<()>) -> Result<&mut Self> {
        fn_(self.brush)?;
        Ok(self)
    }

    pub fn replace(&mut self, brush: &'a mut SolidBrush) -> &mut Self {
        self.brush = brush;
        self
    }

    pub fn fill_rectangle(
        &mut self,
        position: Point,
        width: REAL,
        height: REAL,
    ) -> Result<&mut Self> {
        return_iferror!(GdipFillRectangle(
            self.graphics.graphics(),
            self.brush.brush(),
            position.0,
            position.1,
            width,
            height,
        ));

        Ok(self)
    }

    pub fn fill_rectangles(
        &mut self,
        rects: Vec<GpRectF>,
    ) -> Result<&mut Self> {
        return_iferror!(GdipFillRectangles(
            self.graphics.graphics(),
            self.brush.brush(),
            rects.as_ptr(),
            rects.len() as i32,
        ));

        Ok(self)
    }
}
