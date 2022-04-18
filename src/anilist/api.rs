use serde::Deserialize;
use serde_with::{serde_as, DefaultOnNull};

#[derive(Clone, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Result {
    Media { data: MediaData },
    MediaList { data: MediaListData },
    Error { errors: Vec<Error> },
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Error {
    pub(crate) status: u16,
    pub(crate) message: String,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct MediaData {
    pub(crate) page: Page,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Page {
    pub(crate) page_info: PageInfo,
    pub(crate) media: Vec<MediaIdAndRelation>,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PageInfo {
    pub(crate) has_next_page: bool,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct MediaListData {
    pub(crate) media_list_collection: MediaListCollection,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaListCollection {
    pub(crate) lists: Vec<MediaListGroup>,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaListGroup {
    pub(crate) entries: Vec<MediaList>,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaList {
    pub(crate) media: Media,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaIdAndRelation {
    pub(crate) id: i32,
    pub(crate) relations: MediaConnection,
}

#[serde_as]
#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Media {
    pub(crate) id: i32,
    pub(crate) title: MediaTitle,
    #[serde_as(as = "DefaultOnNull")]
    pub(crate) season_int: i32,
    pub(crate) relations: MediaConnection,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaTitle {
    pub(crate) user_preferred: String,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaConnection {
    pub(crate) edges: Vec<MediaEdge>,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaEdge {
    pub(crate) relation_type: MediaRelation,
    pub(crate) node: MediaId,
}

#[derive(Clone, Deserialize)]
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

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaId {
    pub(crate) id: i32,
}
