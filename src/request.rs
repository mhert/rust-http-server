
pub struct RawHttpRequest {
    pub version: Option<Box<[u8]>>,
    pub method: Option<Box<[u8]>>,
    pub request_uri: Option<Box<[u8]>>,
}
