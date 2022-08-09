use reqwest::header::*;

fn main(){   
    let c = SpoofedMobileClient{
        token : Token{},
    };

    c.login(SwissIDCredentials{
         username : String::from("schnipper@gmail.com"),
         password : String::from("")
        });

    println!("{:?}",c);
    
}

#[derive(Debug)]
struct SwissIDCredentials {
    username:String,
    password:String
}

#[derive(Debug)]
struct SpoofedMobileClient{
    token:Token,
}

impl SpoofedMobileClient{

    const REDIRECT_URL:&str = "https://ch.post.pcc://auth/1016c75e-aa9c-493e-84b8-4eb3ba6177ef";
    const CIENT_ID:&str="ae9b9894f8728ca78800942cda638155";
    const SECRETE:&str="89ff451ede545c3f408d792e8caaddf0";

    fn login(&self,credentials:SwissIDCredentials){

        let mut headers = HeaderMap::new();
        
        let user_agent:&str = concat!(
                "'Mozilla/5.0 (Linux; Android 6.0.1; wv)' ",
                "'AppleWebKit/537.36 (KHTML, like Gecko) ",
                "'Version/4.0 Chrome/52.0.2743.98 Mobile Safari/537.36''"
            );
        
        let client = reqwest::blocking::Client::builder()
            .user_agent(user_agent)
            .build()
            .unwrap();

        let response = client.get("https://pccweb.api.post.ch/OAuth/authorization")
            .send();
            println!("Response: {:?}",response);
        
    }
    
}
    
    

#[derive(Debug)]
struct Token { } 

impl Token { 

}



