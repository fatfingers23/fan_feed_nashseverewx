use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type Root = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    pub id: i64,
    pub date: String,
    #[serde(rename = "date_gmt")]
    pub date_gmt: String,
    pub guid: Guid,
    pub modified: String,
    #[serde(rename = "modified_gmt")]
    pub modified_gmt: String,
    pub slug: String,
    pub status: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub link: String,
    pub title: Title,
    pub content: Content,
    pub excerpt: Excerpt,
    pub author: i64,
    #[serde(rename = "featured_media")]
    pub featured_media: i64,
    #[serde(rename = "comment_status")]
    pub comment_status: String,
    #[serde(rename = "ping_status")]
    pub ping_status: String,
    pub sticky: bool,
    pub template: String,
    pub format: String,
    pub meta: Meta,
    pub categories: Vec<i64>,
    pub tags: Vec<Value>,
    pub acf: Vec<Value>,
    #[serde(rename = "yoast_head")]
    pub yoast_head: String,
    #[serde(rename = "yoast_head_json")]
    pub yoast_head_json: YoastHeadJson,
    #[serde(rename = "jetpack_featured_media_url")]
    pub jetpack_featured_media_url: String,
    #[serde(rename = "jetpack_sharing_enabled")]
    pub jetpack_sharing_enabled: bool,
    #[serde(rename = "jetpack_shortlink")]
    pub jetpack_shortlink: String,
    #[serde(rename = "_links")]
    pub links: Links,
    #[serde(rename = "_embedded")]
    pub embedded: Embedded,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Guid {
    pub rendered: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub rendered: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub rendered: String,
    pub protected: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Excerpt {
    pub rendered: String,
    pub protected: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(rename = "_acf_changed")]
    pub acf_changed: bool,
    #[serde(rename = "jetpack_post_was_ever_published")]
    pub jetpack_post_was_ever_published: bool,
    #[serde(rename = "_jetpack_newsletter_access")]
    pub jetpack_newsletter_access: String,
    #[serde(rename = "_jetpack_dont_email_post_to_subs")]
    pub jetpack_dont_email_post_to_subs: bool,
    #[serde(rename = "_jetpack_newsletter_tier_id")]
    pub jetpack_newsletter_tier_id: i64,
    pub footnotes: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YoastHeadJson {
    pub title: String,
    pub robots: Robots,
    pub canonical: String,
    #[serde(rename = "og_locale")]
    pub og_locale: String,
    #[serde(rename = "og_type")]
    pub og_type: String,
    #[serde(rename = "og_title")]
    pub og_title: String,
    #[serde(rename = "og_description")]
    pub og_description: String,
    #[serde(rename = "og_url")]
    pub og_url: String,
    #[serde(rename = "og_site_name")]
    pub og_site_name: String,
    #[serde(rename = "article_published_time")]
    pub article_published_time: String,
    #[serde(rename = "article_modified_time")]
    pub article_modified_time: Option<String>,
    #[serde(rename = "og_image")]
    pub og_image: Vec<OgImage>,
    pub author: String,
    #[serde(rename = "twitter_card")]
    pub twitter_card: String,
    #[serde(rename = "twitter_creator")]
    pub twitter_creator: String,
    #[serde(rename = "twitter_site")]
    pub twitter_site: String,
    #[serde(rename = "twitter_misc")]
    pub twitter_misc: TwitterMisc,
    pub schema: Schema,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Robots {
    pub index: String,
    pub follow: String,
    #[serde(rename = "max-snippet")]
    pub max_snippet: String,
    #[serde(rename = "max-image-preview")]
    pub max_image_preview: String,
    #[serde(rename = "max-video-preview")]
    pub max_video_preview: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgImage {
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub url: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitterMisc {
    #[serde(rename = "Written by")]
    pub written_by: String,
    #[serde(rename = "Est. reading time")]
    pub est_reading_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@graph")]
    pub graph: Vec<Graph>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Graph {
    #[serde(rename = "@type")]
    pub type_field: String,
    #[serde(rename = "@id")]
    pub id: String,
    pub is_part_of: Option<IsPartOf>,
    pub author: Option<Author>,
    pub headline: Option<String>,
    pub date_published: Option<String>,
    pub date_modified: Option<String>,
    pub main_entity_of_page: Option<MainEntityOfPage>,
    pub word_count: Option<i64>,
    pub comment_count: Option<i64>,
    pub publisher: Option<Publisher>,
    #[serde(default)]
    pub article_section: Vec<String>,
    pub in_language: Option<String>,
    #[serde(default)]
    pub potential_action: Vec<PotentialAction>,
    pub url: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub logo: Option<Logo>,
    pub image: Option<Image>,
    pub same_as: Option<Vec<String>>,
    pub breadcrumb: Option<Breadcrumb>,
    pub item_list_element: Option<Vec<ItemListElement>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsPartOf {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainEntityOfPage {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Publisher {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PotentialAction {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub target: Value,
    #[serde(rename = "query-input")]
    pub query_input: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Logo {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub in_language: String,
    #[serde(rename = "@id")]
    pub id: String,
    pub url: String,
    pub content_url: String,
    pub width: i64,
    pub height: i64,
    pub caption: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    #[serde(rename = "@type")]
    pub type_field: Option<String>,
    pub in_language: Option<String>,
    #[serde(rename = "@id")]
    pub id: String,
    pub url: Option<String>,
    pub content_url: Option<String>,
    pub caption: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Breadcrumb {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemListElement {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub position: i64,
    pub name: String,
    pub item: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: Vec<Self_field>,
    pub collection: Vec<Collection>,
    pub about: Vec<About>,
    pub author: Vec<Author2>,
    pub replies: Vec<Reply>,
    #[serde(rename = "version-history")]
    pub version_history: Vec<VersionHistory>,
    #[serde(rename = "predecessor-version")]
    #[serde(default)]
    pub predecessor_version: Vec<PredecessorVersion>,
    #[serde(rename = "wp:attachment")]
    pub wp_attachment: Vec<WpAttachment>,
    #[serde(rename = "wp:term")]
    pub wp_term: Vec<WpTerm>,
    pub curies: Vec<Cury>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Self_field {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct About {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author2 {
    pub embeddable: bool,
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reply {
    pub embeddable: bool,
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionHistory {
    pub count: i64,
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PredecessorVersion {
    pub id: i64,
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WpAttachment {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WpTerm {
    pub taxonomy: String,
    pub embeddable: bool,
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cury {
    pub name: String,
    pub href: String,
    pub templated: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Embedded {
    pub author: Vec<Author3>,
    #[serde(rename = "wp:term")]
    pub wp_term: Vec<Vec<WpTerm2>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author3 {
    pub code: String,
    pub message: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub status: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WpTerm2 {
    pub id: i64,
    pub link: String,
    pub name: String,
    pub slug: String,
    pub taxonomy: String,
    #[serde(rename = "yoast_head")]
    pub yoast_head: String,
    #[serde(rename = "yoast_head_json")]
    pub yoast_head_json: YoastHeadJson2,
    pub acf: Vec<Value>,
    #[serde(rename = "_links")]
    pub links: Links2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YoastHeadJson2 {
    pub title: String,
    pub robots: Robots2,
    pub canonical: String,
    #[serde(rename = "og_locale")]
    pub og_locale: String,
    #[serde(rename = "og_type")]
    pub og_type: String,
    #[serde(rename = "og_title")]
    pub og_title: String,
    #[serde(rename = "og_url")]
    pub og_url: String,
    #[serde(rename = "og_site_name")]
    pub og_site_name: String,
    #[serde(rename = "og_image")]
    pub og_image: Vec<OgImage2>,
    #[serde(rename = "twitter_card")]
    pub twitter_card: String,
    #[serde(rename = "twitter_site")]
    pub twitter_site: String,
    pub schema: Schema2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Robots2 {
    pub index: String,
    pub follow: String,
    #[serde(rename = "max-snippet")]
    pub max_snippet: String,
    #[serde(rename = "max-image-preview")]
    pub max_image_preview: String,
    #[serde(rename = "max-video-preview")]
    pub max_video_preview: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgImage2 {
    pub width: i64,
    pub height: i64,
    pub url: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema2 {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@graph")]
    pub graph: Vec<Graph2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Graph2 {
    #[serde(rename = "@type")]
    pub type_field: String,
    #[serde(rename = "@id")]
    pub id: String,
    pub url: Option<String>,
    pub name: Option<String>,
    pub is_part_of: Option<IsPartOf2>,
    pub breadcrumb: Option<Breadcrumb2>,
    pub in_language: Option<String>,
    pub item_list_element: Option<Vec<ItemListElement2>>,
    pub description: Option<String>,
    pub publisher: Option<Publisher2>,
    #[serde(default)]
    pub potential_action: Vec<PotentialAction2>,
    pub logo: Option<Logo2>,
    pub image: Option<Image2>,
    pub same_as: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsPartOf2 {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Breadcrumb2 {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemListElement2 {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub position: i64,
    pub name: String,
    pub item: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Publisher2 {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PotentialAction2 {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub target: Target,
    #[serde(rename = "query-input")]
    pub query_input: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub url_template: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Logo2 {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub in_language: String,
    #[serde(rename = "@id")]
    pub id: String,
    pub url: String,
    pub content_url: String,
    pub width: i64,
    pub height: i64,
    pub caption: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image2 {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links2 {
    #[serde(rename = "self")]
    pub self_field: Vec<Self_field2>,
    pub collection: Vec<Collection2>,
    pub about: Vec<About2>,
    #[serde(rename = "wp:post_type")]
    pub wp_post_type: Vec<WpPostType>,
    pub curies: Vec<Cury2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Self_field2 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection2 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct About2 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WpPostType {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cury2 {
    pub name: String,
    pub href: String,
    pub templated: bool,
}
