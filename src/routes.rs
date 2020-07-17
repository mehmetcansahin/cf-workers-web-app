use crate::controllers::home::{hello_user, index};
use crate::utils::{json_response, Params, ResponseData};
use path_tree::PathTree;
use web_sys::{Request, Response, Url};

pub fn api_tree() -> PathTree<Box<dyn Fn(Request, Params) -> Response>> {
    let mut tree = PathTree::<Box<dyn Fn(Request, Params) -> Response>>::new();
    tree.insert("/GET/", Box::new(index));
    tree.insert("/GET/hello/:username", Box::new(hello_user));
    tree
}

pub fn run(request: Request) -> Response {
    let tree = api_tree();

    let url = Url::new(&request.url()).unwrap();
    let path = "/".to_owned() + request.method().as_str() + &url.pathname();
    match tree.find(&path) {
        Some((node, params)) => {
            let params = params
                .iter()
                .map(|p| (p.0.to_string(), p.1.to_string()))
                .collect::<Params>();
            node(request, params)
        }
        None => {
            let response_data = ResponseData {
                code: 404,
                status: "not_found".to_string(),
                message: "404 NOT FOUND".to_string(),
                ..Default::default()
            };
            json_response(response_data)
        }
    }
}
