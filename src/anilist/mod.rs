mod api;

use anyhow::{bail, Result};
use lazy_static::lazy_static;
use petgraph::{algo::tarjan_scc, graphmap::UnGraphMap};
use reqwest::{
    header::{HeaderMap, ACCEPT, CONTENT_TYPE},
    Client,
};
use serde_json::json;
use std::collections::HashSet;

pub(crate) struct Franchise {
    pub(crate) title: String,
    pub(crate) entries: Vec<Media>,
}

pub(crate) struct Media {
    pub(crate) title: String,
}

impl From<&api::Media> for Media {
    fn from(media: &api::Media) -> Self {
        Self {
            title: media.title.user_preferred.to_owned(),
        }
    }
}

static MEDIA_QUERY: &str = "
    query ($ids: [Int], $page: Int) {
        Page (page: $page, perPage: 50) {
            pageInfo {
                hasNextPage
            }
            media (id_in: $ids) {
                id
                relations {
                    edges {
                        relationType (version: 2)
                        node {
                            id
                        }
                    }
                }
            }
        }
    }
";

static MEDIA_LIST_QUERY: &str = "
    query ($userName: String) {
        MediaListCollection (userName: $userName, type: ANIME, status: COMPLETED) {
            lists {
                entries {
                    media {
                        id
                        title {
                            userPreferred
                        }
                        seasonInt
                        relations {
                            edges {
                                relationType (version: 2)
                                node {
                                    id
                                }
                            }
                        }
                    }
                }
            }
        }
    }
";

lazy_static! {
    static ref HEADERS: HeaderMap = {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers
    };
}

pub(crate) struct AniList {
    client: Client,
    username: String,
}

impl AniList {
    pub(crate) fn new(username: &str) -> Self {
        AniList {
            client: Client::new(),
            username: username.to_string(),
        }
    }

    pub(crate) async fn get_franchises(&self) -> Result<Vec<Franchise>> {
        let mut media_list = self.get_media_list().await?;
        let mut visited_ids = media_list
            .iter()
            .map(|media| media.id)
            .collect::<HashSet<i32>>();
        let mut franchise_graph = UnGraphMap::from_edges(media_list.iter().flat_map(|media| {
            media.relations.edges.iter().filter_map(|relation| {
                return match relation.relation_type {
                    api::MediaRelation::Prequel
                    | api::MediaRelation::Sequel
                    | api::MediaRelation::Parent
                    | api::MediaRelation::SideStory
                    | api::MediaRelation::Summary
                    | api::MediaRelation::Alternative
                    | api::MediaRelation::SpinOff => Some((media.id, relation.node.id)),
                    _ => None,
                };
            })
        }));

        loop {
            let ids = &franchise_graph.nodes().collect() - &visited_ids;

            if ids.len() == 0 {
                break;
            }

            let mut page = 1;

            loop {
                let body = json!({
                    "query": MEDIA_QUERY,
                    "variables": {
                        "ids": ids,
                        "page": page,
                    }
                });

                let res = self
                    .client
                    .post("https://graphql.anilist.co/")
                    .headers((*HEADERS).to_owned())
                    .json(&body)
                    .send()
                    .await?
                    .json::<api::Result>()
                    .await?;

                match &res {
                    api::Result::Media { data } => {
                        for media in &data.page.media {
                            for relation in media.relations.edges.iter().filter(|relation| {
                                return match relation.relation_type {
                                    api::MediaRelation::Prequel
                                    | api::MediaRelation::Sequel
                                    | api::MediaRelation::Parent
                                    | api::MediaRelation::SideStory
                                    | api::MediaRelation::Summary
                                    | api::MediaRelation::Alternative
                                    | api::MediaRelation::SpinOff => true,
                                    _ => false,
                                };
                            }) {
                                franchise_graph.add_edge(media.id, relation.node.id, ());
                            }
                        }

                        if !data.page.page_info.has_next_page {
                            break;
                        }

                        page += 1;
                    }
                    api::Result::Error { errors } => {
                        if let Some(error) = errors.iter().next() {
                            bail!("{}: {}", error.status, error.message);
                        }
                    }
                    _ => {
                        bail!("error decoding response body");
                    }
                }
            }

            visited_ids.extend(&ids);
        }

        media_list.sort_by_key(|media| media.season_int);

        Ok(tarjan_scc(&franchise_graph)
            .iter()
            .map(|franchise| {
                let entries = media_list
                    .iter()
                    .filter_map(|media| {
                        if franchise.contains(&media.id) {
                            Some(Media::from(media))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Media>>();

                Franchise {
                    title: entries
                        .iter()
                        .map(|entry| entry.title.to_owned())
                        .next()
                        .unwrap_or_default(),
                    entries,
                }
            })
            .collect())
    }

    async fn get_media_list(&self) -> Result<Vec<api::Media>> {
        let body = json!({
            "query": MEDIA_LIST_QUERY,
            "variables": {
                "userName": self.username
            }
        });

        let res = self
            .client
            .post("https://graphql.anilist.co/")
            .headers((*HEADERS).to_owned())
            .json(&body)
            .send()
            .await?
            .json::<api::Result>()
            .await?;

        match res {
            api::Result::MediaList { data } => {
                if let Some(media_list) = data.media_list_collection.lists.iter().next() {
                    return Ok(media_list
                        .entries
                        .iter()
                        .map(|entry| entry.media.clone())
                        .collect());
                }
            }
            api::Result::Error { errors } => {
                if let Some(error) = errors.iter().next() {
                    bail!("{}: {}", error.status, error.message);
                }
            }
            _ => {}
        }

        bail!("error decoding response body");
    }
}
