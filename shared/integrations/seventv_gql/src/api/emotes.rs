use crate::http_client::{HttpClient, HttpClientAuthed};

#[derive(Clone)]
pub struct EmoteService<C> {
    client: C,
}

impl<C: HttpClient> EmoteService<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }
}

impl<C: HttpClientAuthed> EmoteService<C> {}
