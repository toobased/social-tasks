use std::time::Duration;

use clap::Arg;

pub enum TaskType {
    Browser,
    Regular
}

impl std::fmt::Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Browser => write!(f, "Browser"),
            Self::Regular => write!(f, "Regular"),
        }
    }
}


#[derive(Debug)]
pub struct AppConfig {
    pub args: AppArgs,
    pub browser_sleep: Duration,
    pub regular_sleep: Duration
}

impl AppConfig {
    pub fn new (args:   AppArgs) -> Self {
        Self {
            args,
            // TODO grap from args?
            browser_sleep: Duration::from_secs(5),
            regular_sleep: Duration::from_secs(2)
        }
    }

    pub fn need_sleep(&self, task_type: &TaskType) -> Duration {
        match task_type {
            TaskType::Browser => self.browser_sleep,
            TaskType::Regular => self.regular_sleep
        }
    }
}

#[derive(Debug)]
pub struct AppArgs {
    pub run_browser: bool,
    pub run_regular: bool,
    pub browser_threads: i32,
}

impl From<clap::ArgMatches> for AppArgs {
    fn from(args: clap::ArgMatches) -> Self {
        let run_browser = args.contains_id("browser");
        let run_regular = args.contains_id("regular");
        let b_threads: &String = args.get_one("browser-threads").unwrap();
            // .unwrap_or(&"1".to_string());
        let browser_threads = b_threads.parse::<i32>()
            .expect("Cant parse browser_threads to number. Invalid input");
        Self {
            run_browser,
            run_regular,
            browser_threads
        }
    }
}

pub fn parse_args () -> AppConfig {
    let matches = clap::App::new("Tasks runner")
        .version("0.1.0")
        .about("Runs bots core tasks")
        .arg(Arg::new("browser")
            .long("browser")
            .takes_value(false)
            .help("If run browser-specific tasks"))
        .arg(Arg::new("regular")
            .long("regular")
            .takes_value(false)
            .help("If run regular tasks"))
        .arg(Arg::new("browser-threads")
            .long("browser-threads")
            .takes_value(true)
            .default_value("1")
            .help("Browser threads to spawn"))
    .get_matches();
    let args = AppArgs::from(matches);
    AppConfig::new(args)
}
