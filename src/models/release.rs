use serde::Deserialize;



#[derive(Deserialize, Debug)]
pub struct Release {
    pub url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub html_url: String,
    pub id: i64,
    pub tag_name: String,
    pub name: String,
    pub created_at: String,
    pub published_at: String,
    pub assets: Vec<Asset>,
    pub tarball_url: String,
    pub zipball_url: String,
    pub body: String,
}

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub url: String,
    pub id: i64,
    pub name: String,
    pub label: String,
    pub content_type: String,
    pub state: String,
    pub size: i64,
    pub download_count: i64,
    pub created_at: String,
    pub updated_at: String,
    pub browser_download_url: String,
}