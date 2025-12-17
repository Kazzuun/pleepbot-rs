use crate::http_client::{HttpClient, HttpClientAuthed};

#[derive(Clone)]
pub struct PaintService<C> {
    client: C,
}

impl<C: HttpClient> PaintService<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }
}

impl<C: HttpClientAuthed> PaintService<C> {}
