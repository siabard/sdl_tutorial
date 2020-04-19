use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::video::FullscreenType;
use sdl2::EventPump;
use sdl2::TimerSubsystem;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

/// 게임 객체
pub struct Game {
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub timer_subsystem: TimerSubsystem,
    pub canvas: WindowCanvas,
    pub event_pump: EventPump,
    pub running: bool,
    pub fullscreen: bool,
    pub frame_count: u32,
    pub time_fps: u32,
    pub last_frame: u32,
    pub last_time: u32,
    pub fps: u32,
}

impl Game {
    pub fn new() -> Game {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let timer_subsystem = sdl_context.timer().unwrap();
        let window = video_subsystem
            .window("SDL2 tutorial", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        Game {
            sdl_context,
            video_subsystem,
            timer_subsystem,

            canvas,
            event_pump,
            running: true,
            fullscreen: false,
            frame_count: 0,
            time_fps: 0,
            last_frame: 0,
            last_time: 0,
            fps: 0,
        }
    }

    pub fn process_event(&mut self) {
        for event in self.event_pump.poll_iter() {
            return match event {
                Event::Quit { timestamp } => {
                    self.running = false;
                    println!("game ended timestamp -> {}", timestamp);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num9),
                    ..
                } => {
                    self.fullscreen = !self.fullscreen;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.running = false;
                }
                _ => (),
            };
        }
    }

    pub fn render(&mut self) {
        let window = self.canvas.window_mut();

        if self.fullscreen {
            window.set_fullscreen(FullscreenType::True).unwrap();
        } else {
            window.set_fullscreen(FullscreenType::Off).unwrap();
        }

        self.frame_count += 1;
        let timer_fps = self.timer_subsystem.ticks() - self.last_frame;
        if timer_fps < (1000 / 60) {
            self.timer_subsystem.delay(1000 / 60 - timer_fps);
        }
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn update(&mut self) {
        self.last_frame = self.timer_subsystem.ticks();

        if self.last_frame >= self.last_time + 1000 {
            self.last_time = self.last_frame;
            self.fps = self.frame_count;
            self.frame_count = 0;
        }

        println!("fps: {}", self.fps);
    }
}

pub fn main() -> Result<(), String> {
    let mut game = Game::new();

    while game.running {
        game.update();
        game.process_event();

        game.render();
    }

    Ok(())
}
