use gpu::{Color, Point, Rect, TextAlign};

use crate::Video;

#[derive(Debug, Clone)]
pub struct Text<T>
where
    T: AsRef<str>,
{
    text: TextType<T>,
    color: Option<Color>,
    size: Option<f64>,
    align: TextAlign,
    position: Point,
}

#[derive(Debug, Clone)]
enum TextType<T>
where
    T: AsRef<str>,
{
    Static(u64),
    Dynamic(T),
}

impl<T> Text<T>
where
    T: AsRef<str>,
{
    pub const fn new_dynamic(text: T) -> Self {
        Self {
            text: TextType::Dynamic(text),
            color: None,
            size: None,
            align: TextAlign::Left,
            position: Point::zero(),
        }
    }

    pub fn new_static(text: T, video: &mut Video) -> Self {
        Self {
            text: TextType::Static(video.create_text_object(text.as_ref())),
            color: None,
            size: None,
            align: TextAlign::Left,
            position: Point::zero(),
        }
    }

    pub fn mesaure_width(&self, video: &mut Video) -> f64 {
        let mut old_size: Option<f64> = None;

        if let Some(size) = self.size {
            old_size = Some(video.swap_text_size(size));
        }

        let width = match self.text {
            TextType::Static(object_id) => video.mesaure_text(object_id),
            TextType::Dynamic(ref text) => video.measure_string(text.as_ref()),
        };

        if let Some(old_size) = old_size {
            video.set_painter_text_size(old_size);
        }

        width
    }

    pub fn measure_height(&self, video: &mut Video) -> f64 {
        let mut old_size: Option<f64> = None;
        let current_size;

        if let Some(size) = self.size {
            current_size = video.swap_text_size(size);
            old_size = Some(current_size);
        } else {
            current_size = video.get_painter_text_size();
        }

        if let Some(old_size) = old_size {
            video.set_painter_text_size(old_size);
        }

        current_size
    }

    pub fn calc_bounds(&self, video: &mut Video) -> Rect {
        Rect::new(
            self.position.x,
            self.position.y,
            self.position.x + self.mesaure_width(video),
            self.position.y + self.measure_height(video),
        )
    }

    pub fn draw(&self, video: &mut Video) {
        let mut position = self.position;
        let mut old_color: Option<Color> = None;
        let mut old_size: Option<f64> = None;
        let old_align = video.swap_painter_text_align(self.align);

        if let Some(color) = self.color {
            old_color = Some(video.swap_painter_color(color));
        }

        if let Some(size) = self.size {
            old_size = Some(video.swap_text_size(size));
        }

        let text_size = old_size.unwrap_or_else(|| video.get_painter_text_size());
        position.y += text_size;

        match self.text {
            TextType::Static(object_id) => video.draw_text(object_id, position),
            TextType::Dynamic(ref text) => video.draw_string(text.as_ref(), position),
        };

        if let Some(old_color) = old_color {
            video.set_painter_color(old_color);
        }

        if let Some(old_size) = old_size {
            video.set_painter_text_size(old_size);
        }

        video.set_painter_text_align(old_align);
    }

    pub fn with_color(mut self, color: Option<Color>) -> Self {
        self.set_color(color);

        self
    }

    pub fn set_color(&mut self, color: Option<Color>) {
        self.color = color;
    }

    pub fn with_size(mut self, size: Option<f64>) -> Self {
        self.set_size(size);

        self
    }

    pub fn set_size(&mut self, size: Option<f64>) {
        self.size = size;
    }

    pub fn with_align(mut self, align: TextAlign) -> Self {
        self.set_align(align);

        self
    }

    pub fn set_align(&mut self, align: TextAlign) {
        self.align = align;
    }

    pub fn with_position(mut self, position: Point) -> Self {
        self.set_position(position);

        self
    }

    pub fn set_position(&mut self, position: Point) {
        self.position = position;
    }

    pub fn dispose(&self, video: &mut Video) {
        match self.text {
            TextType::Static(object_id) => video.delete_object(object_id),
            TextType::Dynamic(_) => {}
        }
    }
}
