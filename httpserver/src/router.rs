use super::handler::{Handler, PageNotFoundHander, StaticPageHandler, WebServiceHandler};
use http::httprequest::{HttpRequest, Method, Resource};
use http::httpresponse::HttpResonse;
use std::io::prelude::{Read, Write};

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            Method::Get => match &req.resource {
                Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let resp: HttpResonse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp: HttpResonse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let resp: HttpResonse = PageNotFoundHander::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}
