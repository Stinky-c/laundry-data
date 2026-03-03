use async_trait::async_trait;

#[async_trait]
pub trait Connection {}

impl Connection for () {}
