use crate::entities as MegalodonEntities;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Marker {
    notifications: InnerMarker,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct InnerMarker {
    last_read_id: String,
    version: u32,
    #[serde(with = "date_format_without_tz")]
    updated_at: DateTime<Utc>,
    pleroma: PleromaMarker,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct PleromaMarker {
    unread_count: u32,
}

impl From<Marker> for MegalodonEntities::Marker {
    fn from(val: Marker) -> Self {
        MegalodonEntities::Marker {
            home: None,
            notifications: Some(val.notifications.into()),
        }
    }
}

impl From<InnerMarker> for MegalodonEntities::marker::InnerMarker {
    fn from(val: InnerMarker) -> Self {
        MegalodonEntities::marker::InnerMarker {
            last_read_id: val.last_read_id,
            version: val.version,
            updated_at: val.updated_at,
            unread_count: Some(val.pleroma.unread_count),
        }
    }
}

mod date_format_without_tz {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
            .map(|naive| naive.and_utc())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_() {
        let data = r#"{
            "notifications": {
                "last_read_id": "1",
                "version": 2,
                "updated_at": "2020-01-02T03:04:05",
                "pleroma": {
                    "unread_count": 3
                }
            }
        }"#;
        let marker: Marker = serde_json::from_str(data).unwrap();
        assert_eq!(marker.notifications.last_read_id, "1");
        assert_eq!(marker.notifications.version, 2);
        assert_eq!(
            marker.notifications.updated_at.to_string(),
            "2020-01-02 03:04:05 UTC"
        );
        assert_eq!(marker.notifications.pleroma.unread_count, 3);

        let serialized = serde_json::to_string(&marker).unwrap();

        assert_eq!(
            serialized,
            r#"{"notifications":{"last_read_id":"1","version":2,"updated_at":"2020-01-02T03:04:05","pleroma":{"unread_count":3}}}"#
        );
    }
}
