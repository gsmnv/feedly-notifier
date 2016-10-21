use regex::Regex;

use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct UnreadCount {
    pub id: String,
    pub count: u64
}

#[derive(Debug, Deserialize)]
pub struct UnreadCounts {
    #[serde(rename = "unreadcounts")]
    unread_counts: Vec<UnreadCount>
}

impl UnreadCounts {
    pub fn categories(&self) -> HashMap<String, UnreadCount> {
        let mut hash = HashMap::new();

        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"user/[^/]*/category/(.*)").unwrap();
        }

        for unread_count in self.unread_counts.iter() {
            let captures = REGEX.captures(unread_count.id.as_str());

            if let Some(category) = captures.and_then(|captures| captures.at(1)) {
                if category == "global.all" {
                    hash.insert("All".to_string(), unread_count.clone());
                } else if category != "global.uncategorized" {
                    hash.insert(category.to_string(), unread_count.clone());
                }
            }
        }

        hash
    }
}
