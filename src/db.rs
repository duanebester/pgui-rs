use async_std::task;
use gpui::*;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::{Arc, Mutex};

pub struct Database {
    pub client: Option<Arc<Mutex<Pool<Postgres>>>>,
}

pub struct DatabaseModel {
    inner: Model<Database>,
}

impl DatabaseModel {
    pub fn new(cx: &mut WindowContext) {
        let model = cx.new_model(|_cx| Database { client: None });

        cx.set_global::<DatabaseModel>(Self { inner: model });
    }
    pub fn connect(&self, db_url: String, cx: &mut WindowContext) {
        let pool = task::block_on(async {
            PgPoolOptions::new()
                .max_connections(5)
                .connect(&db_url.as_str())
                .await
        })
        .unwrap();
        self.inner.update(cx, |model, _cx| {
            model.client = Some(Arc::new(Mutex::new(pool)));
        });
    }
}

impl Global for DatabaseModel {}
