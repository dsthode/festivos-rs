use crate::data::Data;
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::single_middleware;
use gotham::pipeline::single_pipeline;
use gotham::prelude::*;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::{FromState, State, StateData};
use http::response::Response;
use http::status::StatusCode;
use hyper::body::Body;
use serde::Deserialize;

#[derive(Clone)]
struct ServerData {
    data: Data,
}

impl StateData for ServerData {}

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct PathExtractor {
    year: u32,
    month: u32,
    day: u32,
}

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct QueryStringExtractor {
    loc: String,
}

pub struct Server<'a> {
    data: Data,
    addr: &'a str,
}

impl<'server> Server<'server>
where
    'server: 'static,
{
    pub fn new<'a>(addr: &'a str) -> Server<'a> {
        let data = Data::new().unwrap();
        let server = Server { data: data, addr };
        server
    }

    pub fn run(&self) {
        gotham::start(self.addr, self.router());
    }

    fn router(&self) -> Router {
        let server_data = ServerData {
            data: self.data.clone(),
        };
        let middleware = StateMiddleware::new(server_data);
        let pipeline = single_middleware(middleware);
        let (chain, pipelines) = single_pipeline(pipeline);
        build_router(chain, pipelines, |route| {
            route.get("/").to(Self::index_handler);
            route
                .get("/:year/:month/:day")
                .with_path_extractor::<PathExtractor>()
                .with_query_string_extractor::<QueryStringExtractor>()
                .to(Self::festivo_handler)
        })
    }

    fn index_handler(state: State) -> (State, &'static str) {
        (state, "hello!")
    }

    fn festivo_handler(state: State) -> (State, Response<Body>) {
        let server_data = ServerData::borrow_from(&state);
        let path_params = PathExtractor::borrow_from(&state);
        let query_param = QueryStringExtractor::borrow_from(&state);
        let locs = server_data
            .data
            .get_date(path_params.year, path_params.month, path_params.day);
        let mut code;
        if (query_param.loc.is_empty()) {
            match locs {
                Some(_) => code = StatusCode::OK,
                None => code = StatusCode::NOT_FOUND,
            };
        } else {
            match locs {
                None => code = StatusCode::NOT_FOUND,
                Some(v) => match v.iter().find(|x| **x == query_param.loc) {
                    Some(_) => code = StatusCode::OK,
                    None => code = StatusCode::NOT_FOUND,
                },
            };
        }
        (
            state,
            Response::builder()
                .status(code)
                .body(Body::empty())
                .unwrap(),
        )
    }
}
