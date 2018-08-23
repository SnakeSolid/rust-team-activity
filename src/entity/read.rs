use std::collections::HashMap;
use xml::reader::Result as XmlResult;
use xml::reader::XmlEvent;

use super::Entry;
use super::EntryError;
use super::EntryResult;
use super::Feed;
use super::FeedError;
use super::FeedResult;
use super::Object;
use super::ObjectError;
use super::ObjectResult;

pub fn read_map<I>(element_name: &str, it: &mut I) -> HashMap<String, String>
where
    I: Iterator<Item = XmlResult<XmlEvent>>,
{
    let mut properties = HashMap::with_capacity(8);
    let mut current_name: String = String::new();

    while let Some(result) = it.next() {
        match result {
            Ok(XmlEvent::StartElement {
                ref name,
                ref attributes,
                ..
            }) if name.local_name == "link" =>
            {
                let rel = attributes.iter().find(|e| e.name.local_name == "rel");
                let href = attributes.iter().find(|e| e.name.local_name == "href");

                if let (Some(rel), Some(href)) = (rel, href) {
                    properties.insert(rel.value.clone(), href.value.clone());
                }
            }
            Ok(XmlEvent::StartElement { name, .. }) => {
                current_name = name.local_name.clone();
            }
            Ok(XmlEvent::EndElement { ref name, .. }) if name.local_name == element_name => {
                break;
            }
            Ok(XmlEvent::Characters(text)) => {
                properties.insert(current_name.clone(), text);
            }
            Ok(_) => {}
            Err(err) => {
                error!("Failed to read next event: {}", err);

                break;
            }
        }
    }

    properties
}

fn read_object(properties: &HashMap<String, String>) -> ObjectResult<Object> {
    debug!("parsing object from {:?}", properties);

    match properties.get("object-type").map(|s| s.as_str()) {
        Some("http://activitystrea.ms/schema/1.0/comment") => read_comment(properties),
        Some("http://activitystrea.ms/schema/1.0/file") => read_file(properties),
        Some("http://activitystrea.ms/schema/1.0/person") => read_person(properties),
        Some("http://streams.atlassian.com/syndication/types/changeset") => {
            read_changeset(properties)
        }
        Some("http://streams.atlassian.com/syndication/types/issue") => read_issue(properties),
        Some("http://streams.atlassian.com/syndication/types/repository") => {
            read_repository(properties)
        }
        Some("http://streams.atlassian.com/syndication/types/review") => read_review(properties),
        Some("http://streams.atlassian.com/syndication/types/page") => read_page(properties),
        Some("http://streams.atlassian.com/syndication/types/space") => read_space(properties),
        Some(object_type) => Err(ObjectError::wrong_object_type(object_type)),
        None => Err(ObjectError::MissingObjectType),
    }
}

fn read_comment(properties: &HashMap<String, String>) -> ObjectResult<Object> {
    let id = properties
        .get("id")
        .ok_or_else(|| ObjectError::element_not_found("id"))?;
    let alternate = properties
        .get("alternate")
        .ok_or_else(|| ObjectError::element_not_found("alternate"))?;

    Ok(Object::comment(id, alternate))
}

fn read_file(properties: &HashMap<String, String>) -> ObjectResult<Object> {
    let id = properties
        .get("id")
        .ok_or_else(|| ObjectError::element_not_found("id"))?;
    let title = properties
        .get("title")
        .ok_or_else(|| ObjectError::element_not_found("title"))?;
    let alternate = properties
        .get("alternate")
        .ok_or_else(|| ObjectError::element_not_found("alternate"))?;

    Ok(Object::file(id, title, alternate))
}

fn read_person(properties: &HashMap<String, String>) -> ObjectResult<Object> {
    let name = properties
        .get("name")
        .ok_or_else(|| ObjectError::element_not_found("name"))?;
    let email = properties
        .get("email")
        .ok_or_else(|| ObjectError::element_not_found("email"))?;
    let uri = properties
        .get("uri")
        .ok_or_else(|| ObjectError::element_not_found("uri"))?;
    let photo = properties
        .get("photo")
        .ok_or_else(|| ObjectError::element_not_found("photo"))?;
    let username = properties
        .get("username")
        .ok_or_else(|| ObjectError::element_not_found("username"))?;

    Ok(Object::person(name, email, uri, photo, username))
}

fn read_changeset(properties: &HashMap<String, String>) -> ObjectResult<Object> {
    let id = properties
        .get("id")
        .ok_or_else(|| ObjectError::element_not_found("id"))?;
    let title = properties
        .get("title")
        .ok_or_else(|| ObjectError::element_not_found("title"))?;
    let alternate = properties
        .get("alternate")
        .ok_or_else(|| ObjectError::element_not_found("alternate"))?;

    Ok(Object::changeset(id, title, alternate))
}

fn read_issue(properties: &HashMap<String, String>) -> ObjectResult<Object> {
    let id = properties
        .get("id")
        .ok_or_else(|| ObjectError::element_not_found("id"))?;
    let title = properties
        .get("title")
        .ok_or_else(|| ObjectError::element_not_found("title"))?;
    let summary = properties
        .get("summary")
        .ok_or_else(|| ObjectError::element_not_found("summary"))?;
    let alternate = properties
        .get("alternate")
        .ok_or_else(|| ObjectError::element_not_found("alternate"))?;

    Ok(Object::issue(id, title, summary, alternate))
}

fn read_repository(properties: &HashMap<String, String>) -> ObjectResult<Object> {
    let id = properties
        .get("id")
        .ok_or_else(|| ObjectError::element_not_found("id"))?;
    let title = properties
        .get("title")
        .ok_or_else(|| ObjectError::element_not_found("title"))?;
    let alternate = properties
        .get("alternate")
        .ok_or_else(|| ObjectError::element_not_found("alternate"))?;

    Ok(Object::repository(id, title, alternate))
}

fn read_review(properties: &HashMap<String, String>) -> ObjectResult<Object> {
    let id = properties
        .get("id")
        .ok_or_else(|| ObjectError::element_not_found("id"))?;
    let title = properties
        .get("title")
        .ok_or_else(|| ObjectError::element_not_found("title"))?;
    let summary = properties
        .get("summary")
        .ok_or_else(|| ObjectError::element_not_found("summary"))?;
    let alternate = properties
        .get("alternate")
        .ok_or_else(|| ObjectError::element_not_found("alternate"))?;

    Ok(Object::review(id, title, summary, alternate))
}

fn read_page(properties: &HashMap<String, String>) -> ObjectResult<Object> {
    let id = properties
        .get("id")
        .ok_or_else(|| ObjectError::element_not_found("id"))?;
    let title = properties
        .get("title")
        .ok_or_else(|| ObjectError::element_not_found("title"))?;
    let alternate = properties
        .get("alternate")
        .ok_or_else(|| ObjectError::element_not_found("alternate"))?;

    Ok(Object::page(id, title, alternate))
}

fn read_space(properties: &HashMap<String, String>) -> ObjectResult<Object> {
    let id = properties
        .get("id")
        .ok_or_else(|| ObjectError::element_not_found("id"))?;
    let title = properties
        .get("title")
        .ok_or_else(|| ObjectError::element_not_found("title"))?;
    let alternate = properties
        .get("alternate")
        .ok_or_else(|| ObjectError::element_not_found("alternate"))?;

    Ok(Object::space(id, title, alternate))
}

pub fn read_entry<I>(it: &mut I) -> EntryResult<Entry>
where
    I: Iterator<Item = XmlResult<XmlEvent>>,
{
    debug!("Reading entry");

    let mut author: Option<Object> = None;
    let mut objects: Vec<Object> = Vec::with_capacity(0);
    let mut target: Option<Object> = None;
    let mut verbs: Vec<String> = Vec::with_capacity(1);
    let mut properties = HashMap::with_capacity(8);
    let mut element_name = "entry".into();
    let mut depth = 1;

    while let Some(result) = it.next() {
        let event = result.map_err(EntryError::xml_event_error)?;

        debug!("event = {:?}, depth = {}", event, depth);

        match event {
            XmlEvent::StartElement { ref name, .. } if name.local_name == "entry" => {
                element_name = name.local_name.clone();
                depth = 1;
            }
            XmlEvent::StartElement { ref name, .. } if name.local_name == "author" => {
                debug!("Reading author");

                let properties = read_map(&name.local_name, it);

                author = Some(read_object(&properties).map_err(EntryError::read_object_error)?);
            }
            XmlEvent::StartElement { ref name, .. } if name.local_name == "object" => {
                debug!("Reading object");

                let properties = read_map(&name.local_name, it);

                objects.push(read_object(&properties).map_err(EntryError::read_object_error)?);
            }
            XmlEvent::StartElement { ref name, .. } if name.local_name == "target" => {
                debug!("Reading target");

                let properties = read_map(&name.local_name, it);

                target = Some(read_object(&properties).map_err(EntryError::read_object_error)?);
            }
            XmlEvent::StartElement {
                ref name,
                ref attributes,
                ..
            } if name.local_name == "link" =>
            {
                debug!("Reading link");

                let rel = attributes.iter().find(|e| e.name.local_name == "rel");
                let href = attributes.iter().find(|e| e.name.local_name == "href");

                if let (Some(rel), Some(href)) = (rel, href) {
                    properties.insert(rel.value.clone(), href.value.clone());
                }

                element_name = name.local_name.clone();
                depth += 1;
            }
            XmlEvent::StartElement { name, .. } => {
                debug!("Start element: {}", name.local_name);

                element_name = name.local_name.clone();
                depth += 1;
            }
            XmlEvent::EndElement { name, .. } => {
                debug!("End element: {}", name.local_name);

                depth -= 1;

                if depth == 0 {
                    break;
                }
            }
            XmlEvent::Characters(text) => {
                debug!("Text: {}", text);

                if depth == 2 {
                    if element_name == "verb" {
                        verbs.push(text);
                    } else {
                        properties.insert(element_name.clone(), text);
                    }
                }
            }
            _ => {}
        }
    }

    debug!("Entry properties = {:?}", properties);

    let author = author.ok_or_else(|| EntryError::element_not_found("author"))?;
    let alternate = properties
        .get("alternate")
        .ok_or_else(|| EntryError::element_not_found("alternate"))?;
    let application = properties
        .get("application")
        .ok_or_else(|| EntryError::element_not_found("application"))?;
    let content = properties.get("content");
    let id = properties
        .get("id")
        .ok_or_else(|| EntryError::element_not_found("id"))?;
    let published = properties
        .get("published")
        .ok_or_else(|| EntryError::element_not_found("published"))?;
    let timezone_offset = properties
        .get("timezone-offset")
        .ok_or_else(|| EntryError::element_not_found("timezone-offset"))?;
    let title = properties
        .get("title")
        .ok_or_else(|| EntryError::element_not_found("title"))?;
    let updated = properties
        .get("updated")
        .ok_or_else(|| EntryError::element_not_found("updated"))?;

    debug!("Entry complete");

    Ok(Entry::new(
        author,
        &objects,
        target.as_ref(),
        alternate,
        application,
        content,
        id,
        published,
        timezone_offset,
        title,
        updated,
        &verbs,
    ))
}

pub fn read_feed<I>(it: &mut I) -> FeedResult<Feed>
where
    I: Iterator<Item = XmlResult<XmlEvent>>,
{
    debug!("Reading feed");

    let mut entries: Vec<Entry> = Vec::with_capacity(1);
    let mut properties = HashMap::new();
    let mut element_name = "feed".into();
    let mut depth = 1;

    while let Some(result) = it.next() {
        let event = result.map_err(FeedError::xml_event_error)?;

        debug!("event = {:?}, depth = {}", event, depth);

        match event {
            XmlEvent::StartElement { ref name, .. } if name.local_name == "feed" => {
                element_name = name.local_name.clone();
                depth = 1;
            }
            XmlEvent::StartElement { ref name, .. } if name.local_name == "entry" => {
                entries.push(read_entry(it).map_err(FeedError::read_entry_error)?);
            }
            XmlEvent::StartElement {
                ref name,
                ref attributes,
                ..
            } if name.local_name == "link" =>
            {
                debug!("Reading link");

                let rel = attributes.iter().find(|e| e.name.local_name == "rel");
                let href = attributes.iter().find(|e| e.name.local_name == "href");

                if let (Some(rel), Some(href)) = (rel, href) {
                    properties.insert(rel.value.clone(), href.value.clone());
                }

                element_name = name.local_name.clone();
                depth += 1;
            }
            XmlEvent::StartElement { name, .. } => {
                debug!("Start element: {}", name.local_name);

                element_name = name.local_name.clone();
                depth += 1;
            }
            XmlEvent::EndElement { name, .. } => {
                debug!("End element: {}", name.local_name);

                depth -= 1;

                if depth == 0 {
                    break;
                }
            }
            XmlEvent::Characters(text) => {
                debug!("Text: {}", text);

                if depth == 2 {
                    properties.insert(element_name.clone(), text);
                }
            }
            _ => {}
        }
    }

    debug!("Feed properties = {:?}", properties);

    let id = properties
        .get("id")
        .ok_or_else(|| FeedError::element_not_found("id"))?;
    let title = properties
        .get("title")
        .ok_or_else(|| FeedError::element_not_found("title"))?;
    let timezone_offset = properties
        .get("timezone-offset")
        .ok_or_else(|| FeedError::element_not_found("timezone-offset"))?;
    let updated = properties
        .get("updated")
        .ok_or_else(|| FeedError::element_not_found("updated"))?;

    debug!("Feed complete");

    Ok(Feed::new(id, title, timezone_offset, updated, &entries))
}
