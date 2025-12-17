use crate::http_client::{HttpClient, HttpClientAuthed};

#[derive(Clone)]
pub struct ProductService<C> {
    client: C,
}

impl<C: HttpClient> ProductService<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }
}

impl<C: HttpClientAuthed> ProductService<C> {}
