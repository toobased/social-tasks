use config::{AppConfig, TaskType};
use log::info;
use socials_core::{db::SocialsDb, tasks::{BotTask, BotTaskQuery}};

use crate::config::AppMode;

pub mod config;
#[cfg(test)]
pub mod tests;


pub fn browser_task_query() -> BotTaskQuery {
    let mut query = BotTaskQuery::default();
    query
        .is_active()
        .not_sleep()
        .is_browser()
        .top_old_updated();
    query
}

pub fn regular_task_query() -> BotTaskQuery {
    let mut query = BotTaskQuery::default();
    query
        .is_active()
        .not_sleep()
        .not_browser()
        .top_old_updated();
    query
}

pub async fn get_task(query: &BotTaskQuery, db: &SocialsDb) -> Option<BotTask> {
    SocialsDb::find_one::<BotTask, BotTaskQuery>(query, &db.bots_tasks()).await.unwrap()
}

pub async fn make_task(query: &BotTaskQuery, task_type: &TaskType, db: &SocialsDb) {
    let task = get_task(query, db).await;
    match task {
        Some(mut task) => {
            info!("Will make {} {}", task.id, task.title);
            task.make(db).await;
            task.update_db(&db).await.expect("Cant update task in db");
        }
        None => info!("[{}] No task to make!", task_type)
    }
}

pub async fn loop_task(config: &AppConfig, query: &BotTaskQuery, task_type: &TaskType) {
    let db = match config.args.mode {
        AppMode::Dev => SocialsDb::new_test_instance().await.unwrap(),
        AppMode::Prod => SocialsDb::new_instance().await.unwrap()
    };
    // TODO improve?
    match task_type {
        TaskType::Browser => { if !config.args.run_browser { return } },
        TaskType::Regular => { if !config.args.run_regular { return } },
    };

    info!("Starting loop for {} task", task_type);
    loop {
        make_task(query, task_type, &db).await;
        tokio::time::sleep(config.need_sleep(task_type)).await;
    }
}

pub async fn make_tasks(config: AppConfig) {
    info!("Start warming up...");
    let browser_query = browser_task_query();
    let regular_query = regular_task_query();
    info!("Start making tasks...");
    // let browser_task = tokio::spawn(async move { loop_task(&config, &browser_query).await });
    // TODO make run in parallel
    let browser_task = loop_task(&config, &browser_query, &TaskType::Browser);
    let regular_task = loop_task(&config, &regular_query, &TaskType::Regular);
    let tasks = [browser_task, regular_task];
    futures::future::join_all(tasks).await;
}

#[tokio::main]
async fn main() {
    env_logger::try_init().ok();
    log::set_max_level(log::LevelFilter::Info);
    let config = config::parse_args();
    info!("Config initialized. {:#?}", config);
    // tests::crud_tasks().await;
    make_tasks(config).await;
}
