use gpu::{Color, Gpu, GpuOp, ObjectType, Point, Rect, TextAlign};
use pci::PciBus;
use screen::Screen;

static mut VIDEO: Option<Video> = None;

pub struct Video {
    gpu: Gpu,
    screen: Screen,
    bounds: Rect,
}

impl Video {
    pub fn init(mut self) {
        unsafe {
            self.gpu
                .call_op(GpuOp::Init {
                    width: self.bounds.width() as u64,
                    height: self.bounds.height() as u64,
                })
                .unwrap();

            self.screen.connect(self.gpu.device.mmio.address);

            while !self.screen.is_connected() {}

            VIDEO.replace(self);
        }
    }

    pub fn width(&self) -> f64 {
        self.bounds.width()
    }

    pub fn height(&self) -> f64 {
        self.bounds.height()
    }

    pub fn bounds(&self) -> Rect {
        self.bounds
    }

    pub fn create_text_object(&mut self, text: &str) -> u64 {
        unsafe {
            self.gpu
                .call_op(GpuOp::CreateObject {
                    ty: ObjectType::Text,
                    address: text.as_ptr() as usize,
                    size: 1,
                    length: text.len(),
                })
                .unwrap() as u64
        }
    }

    pub fn delete_object(&mut self, object_id: u64) {
        unsafe {
            self.gpu.call_op(GpuOp::DeleteObject { object_id }).unwrap();
        }
    }

    pub fn set_font_size(&mut self, size: f64) {
        unsafe {
            self.gpu
                .call_op(GpuOp::SetPainterTextSize { size })
                .unwrap();
        }
    }

    pub fn set_painter_color(&mut self, color: Color) {
        unsafe {
            self.gpu.call_op(GpuOp::SetPainterColor { color }).unwrap();
        }
    }

    pub fn get_painter_color(&mut self) -> Color {
        unsafe { self.gpu.call_op(GpuOp::GetPainterColor).unwrap().into() }
    }

    pub fn swap_painter_color(&mut self, new_color: Color) -> Color {
        let old_color = self.get_painter_color();
        self.set_painter_color(new_color);

        old_color
    }

    pub fn swap_text_size(&mut self, new_size: f64) -> f64 {
        let old_size = self.get_painter_text_size();
        self.set_painter_text_size(new_size);

        old_size
    }

    pub fn get_painter_text_size(&mut self) -> f64 {
        unsafe { self.gpu.call_op(GpuOp::GetPainterTextSize).unwrap() }
    }

    pub fn set_painter_text_size(&mut self, size: f64) {
        unsafe {
            self.gpu
                .call_op(GpuOp::SetPainterTextSize { size })
                .unwrap();
        }
    }

    pub fn fill_screen(&mut self, color: Option<Color>) {
        unsafe {
            let mut old_color: Option<Color> = None;

            if let Some(color) = color {
                old_color = Some(self.swap_painter_color(color));
            }

            self.gpu
                .call_op(GpuOp::DrawRect {
                    from: Point::new(0.0, 0.0),
                    width: self.bounds.width(),
                    height: self.bounds.height(),
                })
                .unwrap();

            if let Some(old_color) = old_color {
                self.set_painter_color(old_color)
            }
        }
    }

    pub fn draw_text(&mut self, object_id: u64, position: Point) {
        unsafe {
            self.gpu
                .call_op(GpuOp::DrawText {
                    object_id,
                    position,
                })
                .unwrap();
        }
    }

    pub fn draw_string(&mut self, text: &str, position: Point) {
        unsafe {
            let address = text as *const str as *const u8 as usize;

            self.gpu
                .call_op(GpuOp::DrawString {
                    position,
                    address,
                    length: text.as_bytes().len(),
                })
                .unwrap();
        }
    }

    pub fn measure_string(&mut self, text: &str) -> f64 {
        unsafe {
            let address = text as *const str as *const u8 as usize;

            self.gpu
                .call_op(GpuOp::MesaureString {
                    address,
                    length: text.as_bytes().len(),
                })
                .unwrap()
        }
    }

    pub fn mesaure_text(&mut self, object_id: u64) -> f64 {
        unsafe { self.gpu.call_op(GpuOp::MesaureText { object_id }).unwrap() }
    }

    pub fn set_painter_text_align(&mut self, align: TextAlign) {
        unsafe {
            self.gpu
                .call_op(GpuOp::SetPainterTextAlign { align })
                .unwrap();
        }
    }

    pub fn get_painter_text_align(&mut self) -> TextAlign {
        unsafe { (self.gpu.call_op(GpuOp::GetPainterTextAlign).unwrap() as u8).into() }
    }

    pub fn swap_painter_text_align(&mut self, align: TextAlign) -> TextAlign {
        let old_align = self.get_painter_text_align();
        self.set_painter_text_align(align);

        old_align
    }

    pub fn create_image(&mut self, data: &[u8], width: u64, height: u64) -> u64 {
        unsafe {
            self.gpu
                .call_op(GpuOp::CreateImageObject {
                    width,
                    height,
                    address: data.as_ptr() as usize,
                })
                .unwrap() as u64
        }
    }

    pub fn draw_image(&mut self, object_id: u64, position: Point) {
        unsafe {
            self.gpu
                .call_op(GpuOp::DrawImage {
                    object_id,
                    position,
                })
                .unwrap();
        }
    }

    pub fn draw_image_rect(&mut self, object_id: u64, dst: Rect) {
        unsafe {
            self.gpu
                .call_op(GpuOp::DrawImageRect { object_id, dst })
                .unwrap();
        }
    }

    pub fn flip_buffers(&mut self) {
        unsafe {
            self.gpu.flip_buffers();
        }
    }

    pub fn mut_video() -> &'static mut Option<Video> {
        unsafe { &mut VIDEO }
    }

    pub fn video() -> &'static Option<Video> {
        unsafe { &VIDEO }
    }
}

impl Default for Video {
    fn default() -> Self {
        unsafe {
            let pci = PciBus::default();
            let screen = pci.find_by_id(screen::DEVICE_ID).map(Screen::from).unwrap();
            let gpu = pci.find_by_id(gpu::DEVICE_ID).map(Gpu::from).unwrap();
            let height = screen.height() as f64;
            let width = screen.width() as f64;

            Self {
                gpu,
                screen,
                bounds: Rect::new_from_zero(width, height),
            }
        }
    }
}
