use chrono::Local;
use colored::*;
use std::{
    fs,
    fs::{OpenOptions},
    io::{Read, Write},
    path::Path,
};
use log::{Record, Level, Metadata};

#[derive(Clone)]
pub enum LogLevel {
    Trace,
    Info,
    Warn,
    Error,
    Fatal,
}

#[derive(Clone, PartialEq)]
pub enum Output {
    File {
        file_path: String,
        file_name: String,
    },
    Console,
}

impl Output {
    pub fn new_as_file(file: String) -> Self {
        let  f_settings: Vec<&str> = file.as_str().split_inclusive('/').collect();
        let mut fp = "".to_string();
        for i in 0..(f_settings.len()-1) {
            fp += f_settings.get(i).unwrap();
        }
        Self::File { file_path: fp
            , file_name: f_settings.get(f_settings.len()-1).unwrap().to_string() }
    }
}

struct OutputLogger {
    output: Output,
    name: String,
}

impl OutputLogger {
    // pub fns
    pub fn new(name: String, output: Output) -> Self {
        if let Output::File { file_path: fp, file_name: fname } = output.clone() {
            let path = fp + fname.as_str();
            let real_path = Path::new(path.as_str());
            if real_path.try_exists().unwrap() {
            fs::remove_file(path).unwrap();
            }
        }
        Self {
            output: output,
            name: name,
        }
    }

    pub fn log(&self, level: Level, str: String) {
        let time = Local::now().format("%H:%M:%S");

        let output_level = match level {
            Level::Trace => "Trace".white(),
            Level::Info => "Info".green(),
            Level::Warn => "Warn".yellow(),
            Level::Error => "Error".bright_red(),
            Level::Debug => "Debug".white(),
        };

        let output_level_no_color = match level {
            Level::Trace => "Trace",
            Level::Info => "Info",
            Level::Warn => "Warn",
            Level::Error => "Error",
            Level::Debug => "Debug",
        };
        let output_str = format!("[{}] [{}] [{}]: {}", time, output_level, self.name, str);

        match self.output.clone() {
            Output::Console => {
                println!("{}", output_str);
            }
            Output::File {
                file_path,
                file_name,
            } => {
                let output_str = format!("[{}] [{}] [{}]: {}", time, output_level_no_color, self.name, str);
                self.add_to_file(file_name, file_path, output_str);
            }
        }
    }

    // private fns
    fn make_path_to_file(path: String) {
        let path: Vec<&str> = path.as_str().split_inclusive('/').collect();
        let mut p: String = String::from("");
        for i in path {
            p += &i.to_string();
            p += "/";
            let real_path = Path::new(&p);
            if !real_path.try_exists().unwrap() {
                fs::create_dir(p.clone()).unwrap();
            }
        }
    }

    fn add_to_file(&self, file_name: String, path: String, write_str: String) {
        OutputLogger::make_path_to_file(path.clone());
        let p = path + &file_name;
        let real_path = Path::new(&p);
        if real_path.try_exists().unwrap() {
            // open file
            let mut f = OpenOptions::new()
                .write(true)
                .read(true)
                .create(true)
                .open(p.clone()).unwrap();
            let mut content = String::new();

            // read prev logs
            f.read_to_string(&mut content).unwrap();
            f.write_all(format!("\n{}", write_str).as_bytes()).unwrap();
        } else {
            let mut f = fs::File::create(p).unwrap();
            f.write_all(write_str.as_bytes()).unwrap();
        }
    }
}

pub struct Logger {
    outputs: Vec<OutputLogger>,
    name: String,
}

impl Logger {
    pub fn new(name: String, output: Output) -> Self {
        Self {
             outputs: vec![OutputLogger::new(name.clone(), output)] 
            , name: name}
    }

    pub fn add_output(&mut self, output: Output) {
        self.outputs.push(OutputLogger::new(self.name.clone(), output));
    }

    pub fn delete_output(&mut self, output: Output) {
        for i in 0..self.outputs.len()-1 {
            if output == self.outputs[i].output {
                self.outputs.remove(i);
            }
        }
    }

    pub fn log(&self, level: Level, str: String) {
        for i in self.outputs.iter() {
            i.log(level.clone(), str.clone())
        }
    }
}


impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            for i in self.outputs.iter() {
                i.log(record.level(), record.args().to_string());
            }
        }
    }

    fn flush(&self) {}
}

