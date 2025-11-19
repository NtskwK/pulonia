use tklog::{
    LogOption,
    handle::{FileMixedMode, FileOption},
    sync::Logger,
};

static mut LOGGER: Option<Logger> = None;

pub fn log_init(log_dir: Option<&std::path::Path>) {
    let file_options: Option<Box<dyn FileOption>> = log_dir.map(|dir| {
        Box::new(FileMixedMode::new(
            dir.join("updater.log").to_str().unwrap(),
            10 * 1024 * 1024,
            tklog::MODE::DAY,
            5,
            true,
        )) as Box<dyn FileOption>
    });

    let log_options = LogOption {
        level: Some(tklog::LEVEL::Debug),
        formatter: None,
        format: None,
        console: Some(true),
        fileoption: file_options,
    };

    unsafe {
        let mut logger = Logger::new();
        logger.set_option(log_options);
        LOGGER = Some(logger);
    }
}
