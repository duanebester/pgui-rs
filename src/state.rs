use crate::db;
use gpui::*;
use sqlx::{Pool, Postgres};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Connection {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub user: String,
    pub password: String,
}

#[derive(Clone)]
pub struct State {
    client: Option<Arc<Mutex<Pool<Postgres>>>>,
    connections: Vec<Connection>,
}

pub struct StateModel {
    inner: Model<State>,
}

impl StateModel {
    pub fn new(cx: &mut WindowContext) {
        let model = cx.new_model(|_cx| State {
            client: None,
            connections: vec![], // TODO: Load from app database
        });

        cx.set_global::<StateModel>(Self { inner: model });
    }
}

impl Global for StateModel {}
