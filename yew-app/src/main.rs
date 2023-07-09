use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/curriculumvitae")]
    CurriculumVitae,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(CurriculumVitae)]
fn curriculum_vitae() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Curriculum Vitae" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

#[function_component(Home)]
fn curriculum_vitae() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::CurriculumVitae));
    html! {
        <div>
            <h1>{ "Home" }</h1>
            <button {onclick}>{ "Curriculum Vitae" }</button>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { 
            <Home /> 
        },
        Route::CurriculumVitae => html! {
            <CurriculumVitae />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {

    html! { 
        <BrowserRouter>
        <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}