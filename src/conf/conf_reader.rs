use toml::*;
use std::fs::*;
use std::io::{Read};

#[derive(Deserialize)]
struct ServerSt{
    listenPort: i32,
    UploadUrl: String,
    needpassword: bool
}

#[derive(Deserialize)]
struct OptionSt{
    whitelist : Vec<String>,
    deleteswitch: Option<bool>,
    saveDeleteKeys : Option<bool>,
    idleTimeBound: Option<i64>,
    keyDirName: Option<String>,
    maxDeleteCount: Option<i32>,
    memoryStat: Option<bool>
}

#[derive(Deserialize)]
struct ClusterSt {
    connTimeout : Option<i32>,
    readTimeout : Option<i32>,
    writeTimeout: Option<i32>,
    clusterAddrs: Vec<Vec<String>>,
    clusterPasswords: Vec<String>
}


#[derive(Deserialize)]
struct UtilSt{
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
    fn New(path: &str) -> Result<Self::T, de::Error> ;
}

impl Config for ConfigSt {
    type T = ConfigSt;
    fn New(path: &str) -> Result<Self::T, de::Error> {
        let cFile = File::open(path);

        let mut readed = String::new();
        println!("hello there");
        cFile.unwrap().read_to_string(& mut readed);
        println!("{}", readed);
        let mut k: Self::T = toml::from_str(&mut readed).unwrap();
//        self.replace_none_value(k);
        k.replace_none_value();
        return Ok(k);
    }
}

impl ConfigSt{
    fn replace_none_value(&mut self) {
        if self.util.log_dir == None {
           self.util.log_dir = Some(String::from("./log"));
        }
    }

    pub fn get_log_dir(self) -> String {
        String::from(self.util.log_dir.unwrap())
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

    fn New(path : &str) -> Result<Self::T, de::Error> {
        let cFile = File::open(path);

        let readed = &mut String::new();
        println!("hello there");
        cFile.unwrap().read_to_string(readed);
        println!("{}", readed);
        let k: Self::T = toml::from_str(readed).unwrap();
        return Ok(k);

    }
}

impl ToolConfigSt {
    pub fn get_log_dir(self) -> String {
        String::from(self.util.log_dir.unwrap())
    }
}


//fn replace<T>(old :&mut Option<T>, new :T) where T : std::cmp::PartialEq {
//    if old == None {
//        old = &mut Some(new)
//    }
//}

