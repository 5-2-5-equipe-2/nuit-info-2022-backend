use mysql::{Pool,PooledConn,Opts,Result};

pub struct ConnectInfo {
    pub host : &'static str,
    pub port : &'static str,
    pub user : &'static str,
    pub password : &'static str,
    pub db_name : &'static str
}

pub fn connect_database(connect_info:ConnectInfo) -> Result<Pool> {
    let url = format!("mysql://{}:{}@{}:{}/{}", connect_info.user,connect_info.password,connect_info.host,connect_info.port,connect_info.db_name);
    let pool = Pool::new(Opts::from_url(&url).unwrap());
    pool
}
