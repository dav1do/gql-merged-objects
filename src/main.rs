use async_graphql::{self, Object};

pub fn main() {
    println!("update async-graphql to 4.0.4 to see the SimpleObject error");
}

#[derive(Default)]
pub struct QueryA1 {}

#[derive(Default)]
pub struct QueryB1 {}

#[Object]
impl QueryA1 {
    async fn version(&self) -> async_graphql::Result<String> {
        Ok("0.1.0".into())
    }
}

#[Object]
impl QueryB1 {
    async fn hello(&self) -> async_graphql::Result<String> {
        Ok("hello!".into())
    }
}
