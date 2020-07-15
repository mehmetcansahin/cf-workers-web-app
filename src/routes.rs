use crate::controllers::home::{hello_user, index};
use crate::utils::{Params, ResponseData};
use path_tree::PathTree;
use web_sys::{Request, Response, ResponseInit, Url};

pub fn api_tree() -> PathTree<Box<dyn Fn(Request, Params) -> ResponseData>> {
    let mut tree = PathTree::<Box<dyn Fn(Request, Params) -> ResponseData>>::new();
    tree.insert("/GET/", Box::new(index));
    tree.insert("/GET/hello/:username", Box::new(hello_user));
    tree
}

pub fn run(request: Request) -> Response {
    let tree = api_tree();

    let url = Url::new(&request.url()).unwrap();
    let path = "/".to_owned() + request.method().as_str() + &url.pathname();
    let response_data = match tree.find(&path) {
        Some((node, params)) => {
            let params = params
                .iter()
                .map(|p| (p.0.to_string(), p.1.to_string()))
                .collect::<Params>();
            node(request, params)
        }
        None => ResponseData {
            code: 404,
            status: "not_found".to_string(),
            message: "404 NOT FOUND".to_string(),
            ..Default::default()
        },
    };

    let json = serde_json::to_string(&response_data).unwrap();
    Response::new_with_opt_str_and_init(
        Some(&json),
        ResponseInit::new()
            .headers(
                headers! {
                    "Content-Type" => "application/json",
                    "Cache-Control" => "no-cache"
                }
                .as_ref(),
            )
            .status(response_data.code)
            .as_ref(),
    )
    .unwrap()
}
