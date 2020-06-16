pub struct HttpRequestBody {
    raw: Vec<u8>
}

// TODO: add methods to handle the body.
impl HttpRequestBody {
    pub fn new(raw: Vec<u8>) -> Self {
        HttpRequestBody {
            raw
        }
    }
}
