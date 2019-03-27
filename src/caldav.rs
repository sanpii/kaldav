use crate::Children;
use crate::Requestable;
use std::collections::HashMap;
use std::convert::Into;

pub struct Caldav {
    url: String,
    auth: Option<crate::Authorization>,
}

impl crate::Requestable for Caldav {
    fn get_auth(&self) -> Option<crate::Authorization> {
        self.auth.clone()
    }

    fn set_auth(&mut self, auth: Option<crate::Authorization>) {
        self.auth = auth;
    }
}

impl crate::Xmlable for Caldav {
    fn get_url(&self) -> String {
        self.url.clone()
    }
}

impl crate::Children for Caldav {
    fn new<S>(url: S) -> Self where S: Into<String> {
        Caldav {
            url: url.into(),
            auth: None,
        }
    }
}

impl Caldav {
    pub fn new<S>(url: S) -> Self where S: Into<String> {
        Caldav {
            url: url.into(),
            auth: None,
        }
    }

    pub fn principals(&self) -> crate::result::Result<Vec<crate::principal::Principal>> {
        let response = self.propfind(self.url.clone(), r#"
<d:propfind xmlns:d="DAV:">
    <d:prop>
        <d:current-user-principal />
    </d:prop>
</d:propfind>
"#);

        match response {
            Ok(response) => Ok(self.to_vec(response.as_str(), "//d:current-user-principal/d:href/text()")),
            Err(err) => Err(err),
        }
    }

    fn principal(&self) -> crate::result::Result<crate::principal::Principal> {
        match self.principals() {
            Ok(p) => Ok(p[0].clone()),
            Err(err) => Err(err),
        }
    }

    fn home(&self) -> crate::result::Result<Vec<crate::home::Home>> {
        match self.principal() {
            Ok(principal) => principal.home(),
            Err(err) => Err(err),
        }
    }

    pub fn calendars(&self) -> crate::result::Result<HashMap<String, crate::calendar::Calendar>> {
        match self.home() {
            Ok(home) => home[0].calendars(),
            Err(err) => Err(err),
        }
    }
}
