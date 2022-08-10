use crate::component::verify::Verify;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/verify")]
    Verify,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Verify => {
            html!{
                <Verify/>
            }
            
        },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
