use async_trait::async_trait;

pub struct Event;


#[async_trait]
pub trait EventRepo {

}

pub struct SqlxEventRepo {

}