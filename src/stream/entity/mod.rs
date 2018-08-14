use std::io::Read;
use xml::EventReader;

mod entry;
mod error;
mod feed;
mod object;
mod read;

pub use self::entry::Entry;
pub use self::error::EntryError;
pub use self::error::EntryResult;
pub use self::error::FeedError;
pub use self::error::FeedResult;
pub use self::error::ObjectError;
pub use self::error::ObjectResult;
pub use self::feed::Feed;
pub use self::object::Object;
pub use self::read::read_feed;

pub fn read<R>(read: R) -> FeedResult<Feed>
where
    R: Read,
{
    let mut it = EventReader::new(read).into_iter();

    read_feed(&mut it)
}
