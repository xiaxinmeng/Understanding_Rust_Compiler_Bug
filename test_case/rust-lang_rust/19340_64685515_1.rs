
#[deriving(Clone,Decodable,Encodable,Eq,Hash,PartialEq)]
pub enum Method {
        Get,
        Put,
        Destroy,
        Publish,
        Subscribe,
}

#[deriving(Clone, Encodable, Decodable)] 
pub enum Message {
    PasswordAuth { password: String },
    Resource { method: Method, path: String, payload: Option<String> },
}
