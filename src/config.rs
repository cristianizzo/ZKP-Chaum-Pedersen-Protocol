use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "HOST", default = "127.0.0.1")]
    pub server_address: String,

    #[envconfig(from = "PORT", default = "5000")]
    pub server_port: String,

    #[envconfig(from = "DATABASE_URL", default = "postgres://postgres:password@localhost/nillion")]
    pub database_url: String,

    #[envconfig(from = "MAX_CONNECTIONS", default = "5")]
    pub max_connections: u32,

    #[envconfig(from = "RUST_LOG", default = "trace")]
    pub rust_log: String,

    #[envconfig(from = "DH_P", default = "32317006071311007300338913926423828248817941241140239112842009751400741706634354222619689417363569347117901737909704191754605873209195028853758986185622153212175412514901774520270235796078236248884246189477587641105928646099411723245426622522193230540919037680524235519125679715870117001058055877651038861847280257976054903569732561526167081339361799541336476559160368317896729073178384589680639671900977202194168647225871031411336429319536193471636533209717077448227988588565369208645296636077250268955505928362751121174096972998068410554359584866583291642136218231078990999448652468262416972035911852507045361090559")]
    pub p: String,

    #[envconfig(from = "DH_Q", default = "16158503035655503650169456963211914124408970620570119556421004875700370853317177111309844708681784673558950868954852095877302936604597514426879493092811076606087706257450887260135117898039118124442123094738793820552964323049705861622713311261096615270459518840262117759562839857935058500529027938825519430923620128988027451784866280763083540669680899770668238279580184158948364536589192294840319835950488601097084323612935515705668214659768096735818266604858538724113994294282684604322648318038625134477752964181375560587048486499034205277179792433291645821068109115539495499724326234131208486017955926253522680545279")]
    pub q: String,

    #[envconfig(from = "DH_G", default = "2")]
    pub g: u32,

    #[envconfig(from = "DH_H", default = "4")]
    pub h: u32,
}

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = Config::init_from_env().unwrap();
}