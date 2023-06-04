use gpu::{Point, Rect};

use crate::Video;

#[derive(Debug, Clone)]
pub struct Image {
    bounds: Rect,
    object_id: u64,
}

impl Image {
    pub fn new(data: &'static [u8], bounds: Rect, video: &mut Video) -> Self {
        let object_id = video.create_image(data, bounds.width() as u64, bounds.height() as u64);

        Self { bounds, object_id }
    }

    pub fn with_bounds(mut self, bounds: Rect) -> Self {
        self.set_bounds(bounds);

        self
    }

    pub fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    pub fn draw(&self, video: &mut Video) {
        video.draw_image(self.object_id, self.bounds.position());
    }

    pub fn bounds(&self) -> &Rect {
        &self.bounds
    }

    pub fn translate_x(&mut self, x: f64) {
        self.bounds.translate_x(x);
    }

    pub fn translate_y(&mut self, y: f64) {
        self.bounds.translate_y(y);
    }

    pub fn set_position(&mut self, position: Point) {
        self.bounds.set_position(position);
    }
}
