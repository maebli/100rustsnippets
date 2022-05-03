use reqwest::blocking::Client;
use reqwest::StatusCode;

const HOST_URL:&str = "https://pccweb.api.post.ch/secure/api/mobile/v1dd";
const DEFAULT_HEADER:&str = "'User-Agent': 'Mozilla/5.0 (Linux; Android 6.0.1; wv) AppleWebKit/537.36 (KHTML, like Gecko) '
                          'Version/4.0 Chrome/52.0.2743.98 Mobile Safari/537.36',
            'Authorization': 'Bearer 'format(self.token.token)";

struct SwissID{
   token:u64,
}

struct Request{
    endpoint:String,
    method:String
}


fn main() {

 if let Ok(resp) = reqwest::blocking::get(HOST_URL){
 }

}
