use log::info;
use socials_core::{db::SocialsDb, tasks::{self, BotTaskCreate, TaskActionType, TaskActionEnum, BotTask, BotTaskQuery}, social};

#[tokio::test]
async fn test_tasks () {
    env_logger::init();
    info!("It works")
}

// #[tokio::test]
#[tokio::test]
pub async fn crud_tasks () {
    db_remove_tasks().await;
    db_create_task().await;
}

pub async fn db_remove_tasks() {
    let db = SocialsDb::new_test_instance().await.unwrap();
    SocialsDb::delete_many(
        &BotTaskQuery::default(), &db.bots_tasks()
    ).await.expect("Some error while deleting");
}

pub async fn db_create_task() {
    let db = SocialsDb::new_test_instance().await.unwrap();

    let action = tasks::watch::WatchAction {
        data: tasks::watch::WatchTargetData {
            watch_count: 2,
            watch_seconds: 1,
            resource_link: "https://www.youtube.com/watch?v=HJ-vmXBYIrw".to_string(),
            time_spread: 0,
            ..Default::default()
        },
        ..Default::default()
    };

    let new_task = BotTaskCreate {
        is_active: false,
        title: "testing".to_string(),
        platform: social::SocialPlatform::Youtube,
        is_testing: true,
        // new type
        action_type: TaskActionType::Watch,
        action: TaskActionEnum::WatchAction(action),
        ..Default::default()
    };
    let task: BotTask = BotTask::create_from(&db, new_task).await;
    // println!("task is {:#?}", task);
    SocialsDb::insert_one(task, db.bots_tasks()).await.unwrap();
}
