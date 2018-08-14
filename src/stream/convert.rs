use serde_yaml;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

use config::ActivityConfig;
use config::Config;
use config::IgnoreConfig;
use config::MessageConfig;
use config::MessageGroup;
use stream::Entry;
use stream::Object;

#[derive(Debug)]
pub struct FeedToActivity<'a> {
    ignore: &'a [IgnoreConfig],
    activities: &'a [ActivityConfig],
    messages: &'a [MessageConfig],
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
            let messages = messages.values().into();

            result.insert(group, messages);
        }

        result
    }

    fn get_group_messages(&self, entries: &[Entry]) -> HashMap<String, DistinctGroup<String>> {
        let mut result = HashMap::new();

        for entry in entries {
            if self.should_ingore_entry(entry) {
                continue;
            }

            let keys = self.get_entry_keys(entry);

            if !keys.is_empty() {
                for message in self.messages.iter().filter(|m| keys.contains(m.key())) {
                    if let Some(group) = get_entry_group(entry, message.group()) {
                        let values = result.entry(group).or_insert_with(|| DistinctGroup::new());
                        let messages = message.messages().iter().cloned();

                        values.extend(messages);
                    }
                }
            } else {
                if let Ok(text) = serde_yaml::to_string(entry) {
                    warn!("------- unknown verb list -------");
                    warn!("{}", text);
                    warn!("---------------------------------");
                }
            }
        }

        result
    }

    fn get_entry_keys(&self, entry: &Entry) -> HashSet<&str> {
        let mut keys = HashSet::with_capacity(2);

        for activity in self.activities {
            if is_entry_match(entry, activity.verbs(), activity.application()) {
                keys.insert(activity.key());
            }
        }

        keys
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

    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        iter.into_iter().for_each(|v| self.push(v));
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
        MessageGroup::TargetIssue | MessageGroup::TargetReview => {
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
