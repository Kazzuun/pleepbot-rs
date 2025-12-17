use crate::http_client::{HttpClient, HttpClientAuthed};

#[derive(Clone)]
pub struct BadgeService<C> {
    client: C,
}

impl<C: HttpClient> BadgeService<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }
}

impl<C: HttpClientAuthed> BadgeService<C> {}
