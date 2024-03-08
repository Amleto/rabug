use std::cmp::Ordering;
use serde_derive::Deserialize;

// Defines a simple struct that can encode version information
// We use this to pass around the version of the repclient we're talking to
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Version {
    pub major : u8,
    pub minor : u8,
    pub revision : u8,
}

impl Version {
    pub fn new(major:u8, minor:u8, revision:u8) -> Self {
        Version {
            major,
            minor,
            revision
        }
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Version) -> Ordering {
        (self.major, self.minor, self.revision).cmp(&(other.major, other.minor, other.revision))
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other : &Version) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Version) -> bool {
        (self.major, self.minor, self.revision) == (other.major, other.minor, other.revision)
    }
}

impl Eq for Version {}

