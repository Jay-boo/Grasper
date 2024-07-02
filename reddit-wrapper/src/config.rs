use core::fmt;

#[derive(Clone)]
pub struct Config{
    pub user_agent:String,
    pub client_id:String,
    pub client_secret:String,
    pub username:Option<String>,
    pub password:Option<String>,
    pub access_token:Option<String>,
}

impl Config{
    pub fn new(user_agent:&str,client_id:&str,client_secret:&str)->Config{
        Config{
            user_agent:user_agent.to_string(),
            client_id:client_id.to_string(),
            client_secret:client_secret.to_string(),
            username:None,
            password:None,
            access_token:None
        }
    
    }
}
impl fmt::Display for Config{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"------ Config used :\nCLIENT_ID: {}\nUSER_AGENT: {}\nUSERNAME: {:?}\nAccess Token catch: {}\n--------------",self.client_id,self.user_agent,self.username,self.access_token.is_some())
    }
}

