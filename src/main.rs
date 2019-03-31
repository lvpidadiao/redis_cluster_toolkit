
//extern crate serde_derive;
//extern crate toml;

#[macro_use]
extern crate futures;
extern crate tokio;
extern crate redis;
use crate::conf::conf_reader::Config;


mod genrand;
mod conf;
mod analysis;
mod redisconn;
mod cmdgen;

use tokio::io;
use tokio::net::TcpStream;
use futures::Future;

fn main() {
    let c = conf::conf_reader::ToolConfigSt::New("./conf.toml");
    match c {
        Err(e) => {
        panic!(e)
        }
        Ok(p) => {
//            let c = redisconn::init_single_redis(p.get_first_redis_addr(), p.get_first_redis_password());
            let mut c = redisconn::ConnectionPool::new(p.get_first_redis_addr(), p.get_first_redis_password()).unwrap();
//            let conn = c.get_connection().unwrap();
//            let rst:redis::RedisResult<String> = redis::cmd("get").arg("hello").query(&conn);
            c.iterate::<String>("scan", 0);

            println!("{}", p.get_log_dir())
        }
    }

//    let addr = "127.0.0.1:1234".parse().unwrap();
//
//    let future = TcpStream::connect(&addr)
//        .and_then(|socket| {
//            io::write_all(socket, b"hello world")
//        })
//        .map( |_| println!("write complete!"))
//        .map_err(|_| eprintln!("failed"));
//
//    tokio::run(future);

}

fn check_result() {

}