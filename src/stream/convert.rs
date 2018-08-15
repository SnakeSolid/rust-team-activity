use rand;
use rand::Rng;
use serde_yaml;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

use config::ActivityConfig;
use config::Config;
use config::IgnoreConfig;
use config::MessageGroup;
use stream::Entry;
use stream::Object;

#[derive(Debug)]
pub struct FeedToActivity<'a> {
    ignore: &'a [IgnoreConfig],
    activities: &'a [ActivityConfig],
    messages: &'a HashMap<String, Vec<String>>,
}

impl<'a> FeedToActivity<'a> {
    pub fn new(config: &Config) -> FeedToActivity {
        let activity = config.activity();

        FeedToActivity {
            ignore: activity.ignore(),
            activities: activity.activities(),
            messages: activity.messages(),
        }
    }

    pub fn convert(&self, entries: &[Entry]) -> HashMap<String, Vec<String>> {
        let mut result = HashMap::new();
        let group_messages = self.get_group_messages(entries);

        for (group, messages) in group_messages.into_iter() {
            let messages = messages.values().iter().take(3).cloned().collect();

            result.insert(group, messages);
        }

        result
    }

    fn get_group_messages(&self, entries: &[Entry]) -> HashMap<String, DistinctGroup<String>> {
        let mut result = HashMap::new();
        let mut rng = rand::thread_rng();

        for entry in entries {
            if self.should_ingore_entry(entry) {
                continue;
            }

            let groups = self.get_entry_groups(entry);

            if !groups.is_empty() {
                for (key, group) in groups {
                    if let Some(messages) = self.messages.get(key) {
                        let values = result.entry(group).or_insert_with(|| DistinctGroup::new());

                        if let Some(message) = rng.choose(messages) {
                            values.push(message.clone());
                        }
                    } else {
                        warn!("Messages for key `{}` not found", key);
                    }
                }
            } else {
                if let Ok(text) = serde_yaml::to_string(entry) {
                    warn!("------- unknown verb list -------");
                    warn!("{}", text);
                }
            }
        }

        result
    }

    fn get_entry_groups(&self, entry: &Entry) -> HashMap<&str, String> {
        let mut result = HashMap::with_capacity(2);

        for activity in self.activities {
            if is_entry_match(entry, activity.verbs(), activity.application()) {
                if let Some(group) = get_entry_group(entry, activity.group()) {
                    result.insert(activity.key(), group);
                }
            }
        }

        result
    }

    fn should_ingore_entry(&self, entry: &Entry) -> bool {
        for ignore in self.ignore {
            if is_entry_match(entry, ignore.verbs(), ignore.application()) {
                return true;
            }
        }

        false
    }
}

#[derive(Debug)]
struct DistinctGroup<T>
where
    T: PartialEq + Eq + Hash + Clone,
{
    distinct_values: HashSet<T>,
    sorted_values: Vec<T>,
}

impl<T> DistinctGroup<T>
where
    T: PartialEq + Eq + Hash + Clone,
{
    fn new() -> DistinctGroup<T> {
        DistinctGroup {
            distinct_values: HashSet::new(),
            sorted_values: Vec::new(),
        }
    }

    fn push(&mut self, value: T) {
        if !self.distinct_values.contains(&value) {
            self.distinct_values.insert(value.clone());
            self.sorted_values.push(value);
        }
    }

    fn values(&self) -> &[T] {
        &self.sorted_values
    }
}

fn get_entry_group(entry: &Entry, group: MessageGroup) -> Option<String> {
    match group {
        MessageGroup::TargetIssue | MessageGroup::TargetReview | MessageGroup::TargetPage => {
            entry.target().map(|t| format!("{}", t))
        }
        MessageGroup::ObjectIssue => entry
            .objects()
            .iter()
            .filter_map(|o| {
                if let Object::Issue { .. } = o {
                    Some(format!("{}", o))
                } else {
                    None
                }
            })
            .next(),
        MessageGroup::ObjectReview => entry
            .objects()
            .iter()
            .filter_map(|o| {
                if let Object::Review { .. } = o {
                    Some(format!("{}", o))
                } else {
                    None
                }
            })
            .next(),
        MessageGroup::ObjectPage => entry
            .objects()
            .iter()
            .filter_map(|o| {
                if let Object::Page { .. } = o {
                    Some(format!("{}", o))
                } else {
                    None
                }
            })
            .next(),
        MessageGroup::Content => entry.content().map(|t| find_link(t).into()),
    }
}

fn find_link(text: &str) -> &str {
    if let Some(end_index) = text.find("</a>") {
        if let Some(start_index) = text[..end_index].rfind(">") {
            &text[start_index + 1..end_index]
        } else {
            text
        }
    } else {
        text
    }
}

fn is_entry_match(
    entry: &Entry,
    match_verbs: &[String],
    match_application: Option<&String>,
) -> bool {
    let verbs = entry.verbs();

    if verbs == match_verbs {
        if let Some(application) = match_application {
            if application == entry.application() {
                true
            } else {
                false
            }
        } else {
            true
        }
    } else {
        false
    }
}
