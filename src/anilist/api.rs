use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Result {
    Media { data: MediaData },
    MediaList { data: MediaListData },
    Error { errors: Vec<Error> },
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Error {
    pub(crate) status: u16,
    pub(crate) message: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct MediaData {
    pub(crate) page: Page,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Page {
    pub(crate) page_info: PageInfo,
    pub(crate) media: Vec<Media>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PageInfo {
    pub(crate) has_next_page: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct MediaListData {
    pub(crate) media_list_collection: MediaListCollection,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaListCollection {
    pub(crate) lists: Vec<MediaListGroup>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaListGroup {
    pub(crate) entries: Vec<MediaList>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaList {
    pub(crate) media: MediaWithTitle,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Media {
    pub(crate) id: i32,
    pub(crate) relations: MediaConnection,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaWithTitle {
    pub(crate) id: i32,
    pub(crate) title: MediaTitle,
    pub(crate) relations: MediaConnection,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaTitle {
    pub(crate) user_preferred: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaConnection {
    pub(crate) edges: Vec<MediaEdge>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaEdge {
    pub(crate) relation_type: MediaRelation,
    pub(crate) node: MediaId,
}

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum MediaRelation {
    Adaptation,
    Prequel,
    Sequel,
    Parent,
    SideStory,
    Character,
    Summary,
    Alternative,
    SpinOff,
    Other,
    Source,
    Compilation,
    Contains,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaId {
    pub(crate) id: i32,
}
