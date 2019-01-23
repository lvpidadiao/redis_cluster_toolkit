extern crate redis_cluster_rs;
extern crate redis;
extern crate url;

use redis_cluster_rs::{Client};
use redis::Client as reCli;
use redis::Connection;
use redis::*;
use url::Url;
//use redis::{ConnectionInfo, ConnectionAddr};
pub struct ConnectionPool {
    conn_pool : Vec<Box<Connection>>,
    addr_url : Url,
    redis: reCli,
    check_out: usize,
}

impl ConnectionPool {
    pub fn new(addr: &str, password: &String) -> Option<ConnectionPool> {
        const DSIZE :usize = 5;
        let mut redis_addr = String::from("redis://");

//        password.and_then(|p| {
//            redis_addr.push_str(&p);
//            redis_addr.push('@');
//        });
        if password.ne("") {
            redis_addr.push_str(password);
            redis_addr.push('@');
        }

        redis_addr.push_str(addr);

        let parsed_redis_addr = redis::parse_redis_url(&redis_addr);
        if let Err(e) = parsed_redis_addr {
            println!("can't parse redis addr {:?}, err is {:?}", redis_addr, e);
            return None;
        }

        let pra = parsed_redis_addr.unwrap().clone();

        let tc = reCli::open(pra.clone());

        match tc {
            Ok(t) => {
                let mut cp = ConnectionPool{
                    redis: t,
                    addr_url: pra,
                    check_out: 0,
                    conn_pool: vec![],
                };
                for i in 0..DSIZE {
                    cp.conn_pool.push(Box::new(cp.redis.get_connection().unwrap()))
                }
                Some(cp)
            }
            Err(e) => {
                println!("error occur, {}", e.to_string());
                return None;
            }
        }
    }

    #[inline]
    pub fn get<T:FromRedisValue>(&mut self, cmd:&str, key:&str, result: &mut RedisResult<T>){
        let conn = self.get_conn();
        *result = redis::cmd(cmd).arg(key).query(&*conn);
        self.give_back(conn)
    }

    fn get_conn(&mut self) -> Box<Connection> {
        if self.conn_pool.len() > 0 {
            self.check_out += 1;
            return self.conn_pool.pop().unwrap()
        }else {
            Box::new(self.redis.get_connection().unwrap())
        }
    }

    fn give_back(&mut self, conn :Box<Connection>) {
        if self.conn_pool.len() < 10 {
            self.conn_pool.push(conn);
        }
    }
}

pub fn init_single_redis(addr: &String, password: &String) -> reCli{
    let mut redis_addr = String::from("redis://");

    if password.ne("") {
        redis_addr.push_str(password);
        redis_addr.push('@');
    }


    redis_addr.push_str(addr);
    println!("redis addr is {}", redis_addr);

    reCli::open(redis::parse_redis_url(&redis_addr).unwrap()).unwrap()
}

//fn init_cluster(nodes: &Vec<String>, password: &String) -> Option<Client> {
//    let mut rnodes = Vec::new();
//
//    for element in nodes.iter() {
//        let mut addr = String::from("redis://");
//        addr.push_str(element);
//        rnodes.push(addr);
//    }
//
//    println!("vec size is {}", rnodes.len());
//    let c = Client::open(rnodes).unwrap();
//    c
//}
