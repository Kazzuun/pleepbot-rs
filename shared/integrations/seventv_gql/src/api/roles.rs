use crate::http_client::HttpClient;

#[derive(Clone)]
pub struct RoleService<C> {
    client: C,
}

impl<C: HttpClient> RoleService<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }
}
