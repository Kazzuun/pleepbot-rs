use crate::http_client::{HttpClient, HttpClientAuthed};

#[derive(Clone)]
pub struct EmoteSetService<C> {
    client: C,
}

impl<C: HttpClient> EmoteSetService<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }
}

impl<C: HttpClientAuthed> EmoteSetService<C> {}
