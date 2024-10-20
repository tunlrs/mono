// the input to our `create_user` handler
#[taurpc::ipc_type]
pub struct CreateUser {
    pub username: String,
}

// the output to our `create_user` handler
#[taurpc::ipc_type]
pub struct User {
    pub id: u32,
    pub username: String,
}