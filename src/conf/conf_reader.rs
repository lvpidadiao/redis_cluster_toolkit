use toml::*;
use serde_derive::*;
use std::fs::*;
use std::io::{Read};

#[derive(Deserialize)]
struct ServerSt{
    listen_port: i32,
    upload_url: String,
    needpassword: bool
}

#[derive(Deserialize)]
struct OptionSt{
    whitelist : Option<Vec<String>>,
    delete_switch: Option<bool>,
    save_delete_keys: Option<bool>,
    idle_time_bound: Option<i64>,
    key_dir_name: Option<String>,
    max_delete_count: Option<i32>,
    memory_stat: Option<bool>
}

#[derive(Deserialize)]
struct ClusterSt {
    conn_timeout: Option<i32>,
    read_timeout: Option<i32>,
    write_timeout: Option<i32>,
    cluster_addrs: Vec<Vec<String>>,
    cluster_passwords: Vec<String>
}


#[derive(Deserialize)]
pub struct UtilSt{
    log_dir : Option<String>,
    log_name :Option<String>,
    log_level : String,
}

#[derive(Deserialize)]
pub struct ConfigSt {
    util: UtilSt,
    option : Option<OptionSt>,
    server : ServerSt,
    cluster : ClusterSt
}

#[derive(Deserialize)]
pub struct ToolConfigSt {
    util : UtilSt,
    cluster : ClusterSt,
    option: Option<OptionSt>,
}

//static mut S_CONFIG:ConfigSt = ConfigSt{
//    pattern:String::from("%s"),
//    redis_addrs:Vec::new(),
//    util: UtilSt{
//        log_dir: String::from("./log"),
//        log_name: String::from("info"),
//        log_level : String::from("info"),
//    }
//};


pub trait Config{
    type T;
    fn New(path: &str) -> Result<Self::T, String> ;
}

impl Config for ConfigSt {
    type T = ConfigSt;
    fn New(path: &str) -> Result<Self::T, String> {
        File::open(path)
            .map_err(|err| err.to_string())
            .and_then(|mut file| {
                let mut contents  = String::new();
                file.read_to_string(&mut contents)
                    .map_err(
                        |err| err.to_string()
                    )
                    .map(|_| contents)
            })
            .and_then(|contents| {
                toml::from_str(&contents)
                    .map_err(|err| err.to_string())
                    .and_then( |mut cf:  ConfigSt| {
                        cf.replace_none_value();
                        Ok(cf)
                    })
            })
//        let cFile = File::open(path);
//
//        let mut readed = String::new();
//        println!("hello there");
//        cFile.unwrap().read_to_string(& mut readed);
//        println!("{}", readed);
//        let mut k: Self::T = toml::from_str(&mut readed).unwrap();
//        self.replace_none_value(k);
//        k.replace_none_value();
//        return Ok(k);
    }
}

impl ConfigSt{
    fn replace_none_value(&mut self) {
        if self.util.log_dir == None {
           self.util.log_dir = Some(String::from("./log"));
        }
    }

    pub fn get_log_dir(self) -> String {
//        self.util.log_level.clone()
        self.util.log_dir.unwrap().clone()
//        self.util.log_dir.unwrap()
    }

//    pub fn get_conf_util<'a>(&'a self) ->  &'a UtilSt {
//        &self.util
//    }

    pub fn get_conf_util(&self) ->  &UtilSt {
        &self.util
    }

}

impl Config for ToolConfigSt {
    type T = ToolConfigSt;

    fn New(path : &str) -> Result<Self::T, String> {
        File::open(path)
            .map_err(|err| err.to_string())
            .and_then(|mut file| {
                let mut contents  = String::new();
                file.read_to_string(&mut contents)
                    .map_err(
                        |err| err.to_string()
                    )
                    .map(|_| contents)
            })
            .and_then(|contents| {
                toml::from_str(&contents)
                    .map_err(|err| err.to_string())
            })
    }
}

impl ToolConfigSt {
    pub fn get_log_dir(self) -> String {
        String::from(self.util.log_dir.unwrap())
    }

    pub fn get_first_redis_addr(&self) -> &String {
        &self.cluster.cluster_addrs[0][0]
    }

    pub fn get_first_redis_password(&self) -> &String {
        &self.cluster.cluster_passwords[0]
    }
}


//fn replace<T>(old :&mut Option<T>, new :T) where T : std::cmp::PartialEq {
//    if old == None {
//        old = &mut Some(new)
//    }
//}

