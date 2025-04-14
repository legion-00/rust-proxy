use clap::Parser;


#[derive(Parser, Debug)]
#[command(name = "My Web Server", about = "A custom web server config")]
pub struct Config{

    #[arg(short = 'p' , 
    long = "port" , aliases = ["Port" ,"P"] , default_value_t = 8000 , help= "Port used for ruunning your web-server" )]
    pub port : i32 , 

    #[arg( long = "host" , aliases = ["HOST" , "Host" , "H"] , default_value = "127.0.0.1"  , help = "Address on which the web-wserver will be running")]
    pub host : String ,

    #[arg(short = 't' , long = "target" , aliases = ["Target" , "T" , "tar" , ""] , default_value = "example.com" , help = "This is the target url to which a request will be send" )]
    pub target : String,

    #[arg(long = "read-time" , aliases = ["readtime" , "rtime" , "Readtime" , "ReadTime"] , default_value_t = 10 , help = "Time limit for reading data")]
    pub read_time_out : i16 ,

    #[arg( long = "write-time" , aliases = ["writetime" , "wtime" , "Writetime" , "WriteTime"] , default_value_t = 10 , help = "Time limit for writing to web-server")]
    pub write_time_out : i16 ,

    #[arg( long = "idle-time" , aliases = ["idletime" , "itime" , "Idletime" , "IdleTime"] , default_value_t = 10 , help = "The time for maxminum time limit for an server to stay idle before shutting down")]
    pub idle_time_out : i16 ,

    #[arg( long = "proxy-time" , aliases = ["proxytime" , "ptime" , "Proxytime" , "ProxyTime"] , default_value_t = 10 , help = "The time limit for the web-server alloted for the web-server to connect to a target url")]
    pub proxy_time_out : i16 ,

    #[arg( long = "headersize" , aliases = ["HeaderSize" , "headersize" , "hsize"] , default_value = "127.0.0.1"  , help = "The size of the header allowed" , default_value_t = 1024)]
    pub max_header_bytes : i16 ,

    #[arg( long = "cache--size" , aliases = ["cachesize" , "csize" , "Cache-Size" , "cacheSize" , "CacheSize"] , default_value_t = 1024)]
    pub cache_size : i16 ,

    #[arg( long = "cachetime" , aliases = ["cachetime" , "CacheTime" , "Cache-Time"] , default_value_t = 15)]
    pub cache_time_limit : i16 ,
    
    #[arg( long = "maxconnection" , aliases = ["MaxConnection" , "mconnection" , "Maxconnection"] , default_value_t = 20)]
    pub max_connection : i16 ,
}
