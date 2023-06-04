use core::panic::Location;

use gpu::{Color, TextAlign};

use crate::{StackString, Text, Video};

const FONT_SIZE: f64 = 14.0;
const TEXT_COLOR: Option<Color> = Some(Color::white());
const PANIC_TEXT: &str = "FATAL ERROR! PLEASE, RESTART THE MACHINE.";

pub fn bsod(reason: Option<&str>, location: Option<&Location>) -> ! {
    unsafe {
        let video = Video::mut_video().as_mut().unwrap();

        video.fill_screen(Some(Color::blue()));

        let mut text_pos = video.bounds().center();

        let panic_text = Text::new_static(PANIC_TEXT, video)
            .with_color(TEXT_COLOR)
            .with_size(Some(FONT_SIZE))
            .with_align(TextAlign::Center)
            .with_position(text_pos);

        panic_text.draw(video);

        text_pos.y += FONT_SIZE;

        if let Some(reason) = reason {
            let reason_text = Text::new_dynamic(reason)
                .with_color(TEXT_COLOR)
                .with_size(Some(12.0))
                .with_align(TextAlign::Center)
                .with_position(text_pos);

            reason_text.draw(video);

            text_pos.y += FONT_SIZE;
        }

        if let Some(location) = location {
            let mut msg = StackString::new();
            let mut file = location.file();

            if file.len() > 15 {
                file = &file[file.len() - 15..file.len()];
            }

            msg.format(format_args!(
                "LINE: {}, COLUMN: {}, FILE: ...{}",
                location.line(),
                location.column(),
                file
            ));

            let text = Text::new_dynamic(msg.str())
                .with_color(TEXT_COLOR)
                .with_size(Some(12.0))
                .with_align(TextAlign::Center)
                .with_position(text_pos);

            text.draw(video);
        }

        video.flip_buffers();

        loop {
            riscv::asm::wfi();
        }
    }
}
