use slack_api;

struct AttachmentAuthor {
    name: String,
    link: Option<String>,
    icon: Option<String>,
}

pub struct AttachmentAuthorBuilder {
    name: String,
    link: Option<String>,
    icon: Option<String>,
}

#[allow(dead_code)]
impl AttachmentAuthorBuilder {
    pub fn with_name(name: String) -> Self {
        AttachmentAuthorBuilder {
            name: name,
            link: None,
            icon: None,
        }
    }

    pub fn set_link(mut self, link: String) -> Self {
        self.link = Some(link);
        self
    }

    pub fn set_icon(mut self, icon: String) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn build(self) -> AttachmentAuthor {
        AttachmentAuthor {
            name: self.name,
            link: self.link,
            icon: self.icon,
        }
    }
}

pub struct AttachmentBuilder {
    fallback: String,
    color: Option<String>,
    pretext: Option<String>,
    author: Option<AttachmentAuthor>,
    title: Option<String>,
    title_link: Option<String>,
    text: String,
    fields: Vec<slack_api::api::AttachmentField>,
    image_url: Option<String>,
    thumb_url: Option<String>,
}

#[allow(dead_code)]
impl AttachmentBuilder {
    pub fn with_text_and_fallback(text: String, fallback: String) -> Self {
        AttachmentBuilder {
            fallback: fallback,
            color: None,
            pretext: None,
            author: None,
            title: None,
            title_link: None,
            text: text,
            fields: vec![],
            image_url: None,
            thumb_url: None,
        }
    }

    pub fn set_color<S: Into<String>>(mut self, color: S) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn set_pretext<S: Into<String>>(mut self, pretext: S) -> Self {
        self.pretext = Some(pretext.into());
        self
    }

    pub fn set_author(mut self, author: AttachmentAuthor) -> Self {
        self.author = Some(author);
        self
    }

    pub fn set_title<S: Into<String>>(mut self, title: S, link: Option<S>) -> Self {
        self.title = Some(title.into());
        self.title_link = link.map(|s| s.into());
        self
    }

    pub fn set_image_url<S: Into<String>>(mut self, image_url: S) -> Self {
        self.image_url = Some(image_url.into());
        self
    }

    pub fn set_thumb_url<S: Into<String>>(mut self, thumb_url: S) -> Self {
        self.thumb_url = Some(thumb_url.into());
        self
    }

    pub fn add_field(mut self, field: slack_api::api::AttachmentField) -> Self {
        self.fields.push(field);
        self
    }

    pub fn build(self) -> slack_api::api::Attachment {
        slack_api::api::Attachment {
            fallback: self.fallback,
            color: self.color,
            pretext: self.pretext,
            author_name: self.author.as_ref().map(|a| a.name.clone()),
            author_icon: self.author.as_ref().and_then(|a| a.icon.clone()),
            author_link: self.author.as_ref().and_then(|a| a.link.clone()),
            title: self.title,
            title_link: self.title_link,
            text: self.text,
            fields: if self.fields.is_empty() {
                None
            } else {
                Some(self.fields)
            },
            image_url: self.image_url,
            thumb_url: self.thumb_url,
        }
    }
}
