use crate::http_client::{HttpClient, HttpClientAuthed};

#[derive(Clone)]
pub struct StoreService<C> {
    client: C,
}

impl<C: HttpClient> StoreService<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }
}

impl<C: HttpClientAuthed> StoreService<C> {}
