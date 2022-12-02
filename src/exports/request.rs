use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
/// GET, PUT, POST, etc methods
pub struct Request<'a> {
    _id: String,
    #[serde(rename(deserialize = "parentId", serialize="parentId"))]
    parent_id: String,
    modified: u64,
    created: u64,
    url: String,
    name: &'a str,
    description: &'a str,
    /// GET, POST, etc
    method: &'a str, 
    body: Body,
    parameters: Vec<&'a str>, // not a string
    headers: Vec<&'a str>, // not string
    authentication: Authentication,
    #[serde(rename(deserialize = "metaSortKey", serialize="metaSortKey"))]
    meta_sort_key: i32, // no clue
    #[serde(rename(deserialize = "isPrivate", serialize="isPrivate"))]
    is_private: bool,
    #[serde(rename(deserialize = "settingStoreCookies", serialize="settingStoreCookies"))]
    setting_store_cookies: bool,
    #[serde(rename(deserialize = "settingSendCookies", serialize="settingSendCookies"))]
    setting_send_cookies: bool,
    #[serde(rename(deserialize = "settingDisableRenderRequestBody", serialize="settingDisableRenderRequestBody"))]
    setting_disable_render_request_body: bool,
    #[serde(rename(deserialize = "settingEncodeUrl", serialize="settingEncodeUrl"))]
    setting_encode_url: bool,
    #[serde(rename(deserialize = "settingRebuildPath", serialize="settingRebuildPath"))]
    setting_rebuild_path: bool,
    #[serde(rename(deserialize = "settingFollowRedirects", serialize="settingFollowRedirects"))]
    setting_follow_redirects: &'a str,
    _type: &'a str
}

impl<'a> Request<'a> {
    pub fn new(parent_id: &String, url: String, name: &'a str, description: &'a str, method: &'a str) -> Self {
        let time_since_epoch = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
        let time = time_since_epoch.as_millis() as u64;
        let id = Uuid::new_v4();


        Self {
            _id: id.to_string(),
            parent_id: parent_id.to_string(),
            modified: time,
            created: time,
            url,
            name,
            description,
            method,
            body: Body {  },
            parameters: vec![],
            headers: vec![],
            authentication: Authentication {  },
            meta_sort_key: 00,
            is_private: false,
            setting_store_cookies: false,
            setting_send_cookies: true,
            setting_disable_render_request_body: false,
            setting_encode_url: true,
            setting_rebuild_path: true,
            setting_follow_redirects: "global",
            _type: "request"
        }
    }
 }

#[derive(Serialize)]
 struct Body {

 }

 #[derive(Serialize)]
 struct Authentication {

 }