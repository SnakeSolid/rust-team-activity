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

pub fn read_entry<I>(it: &mut I) -> EntryResult<Entry>
where
    I: Iterator<Item = XmlResult<XmlEvent>>,
{
    info!("Reading entry");

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
                info!("Reading author");

                author = Some(
                    Object::try_from(&read_map(&name.local_name, it))
                        .map_err(EntryError::read_object_error)?,
                );
            }
            XmlEvent::StartElement { ref name, .. } if name.local_name == "object" => {
                info!("Reading object");

                objects.push(
                    Object::try_from(&read_map(&name.local_name, it))
                        .map_err(EntryError::read_object_error)?,
                );
            }
            XmlEvent::StartElement { ref name, .. } if name.local_name == "target" => {
                info!("Reading target");

                target = Some(
                    Object::try_from(&read_map(&name.local_name, it))
                        .map_err(EntryError::read_object_error)?,
                );
            }
            XmlEvent::StartElement {
                ref name,
                ref attributes,
                ..
            } if name.local_name == "link" =>
            {
                info!("Reading link");

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
    info!("Reading feed");

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
                info!("Reading link");

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
