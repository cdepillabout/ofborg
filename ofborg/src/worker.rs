
use amqp::{Consumer, Channel};
use amqp::protocol::basic::{Deliver,BasicProperties};
use std::marker::Send;
use std::io::Error;

pub struct Worker<T: SimpleWorker> {
    internal: T
}

pub struct Response {
}

enum Action {

}

pub trait SimpleWorker {
    type J;
    type R;
    fn consumer(&self, job: &Self::J, resp: Self::R) -> Result<(), Error>;

    fn msg_to_job(&self, method: &Deliver, headers: &BasicProperties,
                  body: &Vec<u8>) -> Result<Self::J, String>;

    fn job_to_actions(&self) -> Self::R;
}

pub fn new<T: SimpleWorker>(worker: T) -> Worker<T> {
    return Worker{
        internal: worker,
    };
}



impl <T: SimpleWorker + Send> Consumer for Worker<T> {
    fn handle_delivery(&mut self,
                       channel: &mut Channel,
                       method: Deliver,
                       headers: BasicProperties,
                       body: Vec<u8>) {

        let job = self.internal.msg_to_job(&method, &headers, &body).unwrap();
        let actions = self.internal.job_to_actions();
        self.internal.consumer(&job, actions).unwrap();
    }
}