use log::{LevelFilter, Log, Metadata, Record};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub struct FileLogger {
    path: PathBuf,
}

impl FileLogger {
    pub fn init() -> Result<(), log::SetLoggerError> {
        let mut path = dirs::data_local_dir().unwrap();
        path.push("UniversalIntelDriver");
        std::fs::create_dir_all(&path).unwrap();
        path.push("logs.txt");
        
        let logger = Box::new(FileLogger { path });
        log::set_boxed_logger(logger)?;
        log::set_max_level(LevelFilter::Info);
        Ok(())
    }
}

impl Log for FileLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .unwrap();
            
        writeln!(file, "[{}] {}: {}", 
            record.level(), 
            record.target(), 
            record.args()
        ).unwrap();
    }

    fn flush(&self) {}
}