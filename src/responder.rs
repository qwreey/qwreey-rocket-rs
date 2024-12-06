use std::sync::Arc;

use qwreey_utility_rs::ElementReadHandle;
use rocket::{
    http::ContentType,
    response::{Responder, Result as RocketResult},
    Request, Response,
};

pub struct ElementResponder<'a> {
    content_type: ContentType,
    body: ElementReadHandle<'a, Arc<str>>,
}
impl<'a> ElementResponder<'a> {
    pub fn new(content_type: ContentType, body: ElementReadHandle<'a, Arc<str>>) -> Self {
        Self { content_type, body }
    }
}
impl<'req, 'res: 'req> Responder<'req, 'res> for ElementResponder<'res> {
    fn respond_to(self, request: &'req Request) -> RocketResult<'res> {
        Ok(Response::build_from(self.body.clone().respond_to(request)?)
            .header(self.content_type)
            .finalize())
    }
}
pub trait ToElementResponder<'a> {
    fn to_element_responder(self, content_type: ContentType) -> ElementResponder<'a>;
}
impl<'a> ToElementResponder<'a> for ElementReadHandle<'a, Arc<str>> {
    fn to_element_responder(self, content_type: ContentType) -> ElementResponder<'a> {
        ElementResponder::new(content_type, self)
    }
}
