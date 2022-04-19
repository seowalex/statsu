use serde::Deserialize;
use std::cmp::Ordering;

#[derive(Clone, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Result {
    Error { errors: Vec<Error> },
    Media { data: MediaData },
    MediaList { data: MediaListData },
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
    pub(crate) media: Vec<MediaIdAndRelations>,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaIdAndRelations {
    pub(crate) id: i32,
    pub(crate) relations: MediaConnection,
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

#[derive(Clone, Copy, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FuzzyDate {
    year: Option<i32>,
    month: Option<i32>,
    day: Option<i32>,
}

impl PartialOrd for FuzzyDate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FuzzyDate {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.year != other.year {
            if self.year.is_some() != other.year.is_some() {
                return other.year.cmp(&self.year);
            }

            return self.year.cmp(&other.year);
        }

        if self.month != other.month {
            if self.month.is_some() != other.month.is_some() {
                return other.month.cmp(&self.month);
            }

            return self.month.cmp(&other.month);
        }

        if self.day.is_some() != other.day.is_some() {
            return other.day.cmp(&self.day);
        }

        self.day.cmp(&other.day)
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
    pub(crate) node: MediaIdAndType,
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
pub(crate) struct MediaIdAndType {
    pub(crate) id: i32,
    pub(crate) r#type: MediaType,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum MediaType {
    Anime,
    Manga,
}
