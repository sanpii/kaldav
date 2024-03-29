use crate::Children;
use crate::Requestable;
use std::convert::Into;

#[derive(Clone, Debug, Default, crate::Object)]
pub struct Principal {
    url: String,
    auth: Option<crate::Authorization>,
}

impl Principal {
    pub fn home(&self) -> crate::Result<crate::Home> {
        let response = self.propfind(
            &self.url,
            r#"
<d:propfind xmlns:d="DAV:" xmlns:c="urn:ietf:params:xml:ns:caldav">
  <d:prop>
     <d:displayname />
     <c:calendar-home-set />
  </d:prop>
</d:propfind>
"#,
        )?;

        self.one(
            &response,
            "//d:response[1]//cal:calendar-home-set/d:href/text()",
        )
        .ok_or_else(|| crate::Error::Misc("No home found".to_string()))
    }
}
