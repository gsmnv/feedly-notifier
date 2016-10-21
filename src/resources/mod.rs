pub mod unread_counts;

pub enum Resource {
    UnreadCounts
}

pub fn resource_to_path(resource: &Resource) -> &'static str {
    match *resource {
        Resource::UnreadCounts => "/v3/markers/counts"
    }
}
