use askama::Template;

#[derive(Template)]
#[template(path = "resume.html")]
pub struct ResumeTemplate {
    pub name: String,
}