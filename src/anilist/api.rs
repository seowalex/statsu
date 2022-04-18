use serde::Deserialize;

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
    pub(crate) media: Vec<Media>,
}

#[derive(Clone, Copy, Deserialize)]
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
pub(crate) struct Media {
    pub(crate) id: i32,
    pub(crate) title: MediaTitle,
    pub(crate) start_date: FuzzyDate,
    pub(crate) relations: MediaConnection,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaTitle {
    pub(crate) user_preferred: String,
}

#[derive(Clone, Copy, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FuzzyDate {
    year: Option<i32>,
    month: Option<i32>,
    day: Option<i32>,
}

impl Default for FuzzyDate {
    fn default() -> Self {
        Self {
            year: Some(i32::MAX),
            month: Some(12),
            day: Some(31),
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaConnection {
    pub(crate) edges: Vec<MediaEdge>,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaEdge {
    pub(crate) relation_type: MediaRelation,
    pub(crate) node: MediaId,
}

#[derive(Clone, Copy, Deserialize)]
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

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaId {
    pub(crate) id: i32,
}
