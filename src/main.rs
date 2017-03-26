mod emulator;

#[macro_use]
extern crate log;
extern crate log4rs;
extern crate rand;

use log::LogLevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

use emulator::Emulator;
use self::emulator::chip8::Chip8;
use self::emulator::window::App;
use self::emulator::romloader;

fn main() {
    configure_logger("config/log4rs.yml".to_string());
    let mut emulator: Box<Chip8> = Box::new(Chip8::new());
    let game = romloader::load_rom("games/game.c8");
    emulator.load(game);
    let mut app: App = App::new(emulator);
    app.run();
}

fn configure_logger(file: String) {
    log4rs::init_file(file, Default::default()).unwrap_or({
        let stdout = ConsoleAppender::builder().build();

        let file = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d} - {l}: {m}{n}")))
            .build("log/rust8.log")
            .unwrap();

        let config = Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .appender(Appender::builder().build("file", Box::new(file)))
            .build(Root::builder().appender("stdout").appender("file").build(LogLevelFilter::Info))
            .unwrap();

        let _ = log4rs::init_config(config);
    });
}