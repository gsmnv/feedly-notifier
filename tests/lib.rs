extern crate feedly_notifier;
extern crate serde_json;

use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

use feedly_notifier::resources::unread_counts::{
    UnreadCounts,
    UnreadCount
};

#[test]
fn unread_counts_categories() {
    let mut file = File::open("./tests/data/unread_counts.json").unwrap();
    let mut json = String::new();
    let mut categories = HashMap::new();

    let all = UnreadCount {
        id: "user/c805fcbf-3acf-4302-a97e-d82f9d7c897f/category/global.all".to_string(),
        count: 605
    };

    let design = UnreadCount {
        id: "user/c805fcbf-3acf-4302-a97e-d82f9d7c897f/category/design".to_string(),
        count: 601
    };

    categories.insert("All".to_string(), all);
    categories.insert("design".to_string(), design);

    file.read_to_string(&mut json).unwrap();

    let unread_counts: UnreadCounts = serde_json::from_str(json.as_str()).unwrap();

    assert_eq!(categories, unread_counts.categories());
}
