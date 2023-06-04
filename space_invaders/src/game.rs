use core::time::Duration;

use gpu::{Color, Point, Rect, TextAlign};
use hid::keyboard::KeyboardKey;

use crate::{io::IoEvent, Image, Io, Text, Time, Video};

macro_rules! include_asset {
    ($path:literal) => {
        include_bytes!(concat!("../assets/", $path))
    };
}

static mut GAME_TITLE_TEXT_OBJECT: Option<Text<&str>> = None;
static mut PRESS_ENTER_LABEL: Option<Text<&str>> = None;

#[derive(Debug, Clone)]
pub struct Game {
    state: GameState,
}

#[derive(Debug, Clone)]
enum GameState {
    MainMenu { next_blink_time: Duration },
    InGame { player: Image },
}

impl Game {
    fn render_main_menu(state: &mut GameState, video: &mut Video) {
        let mut now = Time::now();
        let GameState::MainMenu { next_blink_time } = state else {
            unreachable!()
        };

        video.fill_screen(Some(Color::black()));

        let title = unsafe { GAME_TITLE_TEXT_OBJECT.as_ref().unwrap() };
        title.draw(video);

        let label = unsafe { PRESS_ENTER_LABEL.as_mut().unwrap() };

        if &mut now >= next_blink_time {
            label.set_color(Some(Color::black()));
            *next_blink_time = now + Duration::from_secs(1);
        } else {
            label.set_color(Some(Color::white()));
        }

        label.draw(video);

        video.flip_buffers();
    }

    fn render_in_game(state: &mut GameState, video: &mut Video) {
        let GameState::InGame { player } = state else {
            unreachable!()
        };

        video.fill_screen(Some(Color::black()));

        player.draw(video);

        video.flip_buffers();
    }

    fn render(state: &mut GameState, video: &mut Video) {
        match state {
            GameState::MainMenu { .. } => Self::render_main_menu(state, video),
            GameState::InGame { .. } => Self::render_in_game(state, video),
        }
    }

    fn start_new_game(state: &mut GameState, video: &mut Video) {
        let bounds = video.bounds();

        *state = GameState::InGame {
            player: Image::new(
                include_asset!("player.bitmap"),
                Rect::new_from_zero(11.0, 7.0),
                video,
            )
            .with_bounds(Rect::new_from_position(
                Point::new(bounds.hcenter(), bounds.height() - 22.0),
                11.0,
                7.0,
            )),
        }
    }

    fn handle_main_menu_event(state: &mut GameState, video: &mut Video, ev: IoEvent) {
        let IoEvent::Keyboard(ev) = ev;

        if !matches!(ev.state, hid::KeyState::Down) {
            return;
        }

        if matches!(ev.key, KeyboardKey::Return) {
            Self::start_new_game(state, video);
        }
    }

    fn handle_in_game_event(state: &mut GameState, video: &mut Video, ev: IoEvent) {
        let GameState::InGame { player } = state else {
            unreachable!()
        };

        let IoEvent::Keyboard(ev) = ev;

        if matches!(ev.state, hid::KeyState::Up) {
            return;
        }

        match ev.key {
            KeyboardKey::Left => player.translate_x(-4.0),
            KeyboardKey::Right => player.translate_x(4.0),
            _ => {}
        }
    }

    fn handle_event(state: &mut GameState, video: &mut Video, ev: IoEvent) {
        match state {
            GameState::MainMenu { .. } => Self::handle_main_menu_event(state, video, ev),
            GameState::InGame { .. } => Self::handle_in_game_event(state, video, ev),
        }
    }

    pub fn start(mut self) {
        let video = Video::mut_video().as_mut().unwrap();
        let io = Io::mut_io().as_mut().unwrap();

        unsafe {
            GAME_TITLE_TEXT_OBJECT = Some(
                Text::new_static("SPACE INVADERS", video)
                    .with_align(TextAlign::Center)
                    .with_size(Some(32.0))
                    .with_color(Some(Color::white()))
                    .with_position(video.bounds().center()),
            );

            let title_bounds = GAME_TITLE_TEXT_OBJECT.as_ref().unwrap().calc_bounds(video);

            PRESS_ENTER_LABEL = Some(
                Text::new_static("Press ENTER to start", video)
                    .with_align(TextAlign::Center)
                    .with_size(Some(14.0))
                    .with_color(Some(Color::white()))
                    .with_position(video.bounds().center()),
            );

            let mut label_bounds = PRESS_ENTER_LABEL.as_ref().unwrap().calc_bounds(video);
            label_bounds.translate_y(title_bounds.height() + 12.0);

            PRESS_ENTER_LABEL
                .as_mut()
                .unwrap()
                .set_position(label_bounds.position())
        }

        loop {
            Self::render(&mut self.state, video);

            if let Some(ev) = io.poll() {
                Self::handle_event(&mut self.state, video, ev);
            }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            state: GameState::MainMenu {
                next_blink_time: Duration::from_secs(0),
            },
        }
    }
}
