mod pages;

pub mod yew_prelude {
    pub use yew::prelude::*;
}
use yew_prelude::*;
use yew_router::prelude::*;

use pages::videos::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,

    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(HomePage)]
fn home() -> Html {
    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();

        Callback::from(move |video: Video| {
            selected_video.set(Some(video));
        })
    };

    let details = selected_video.as_ref().map(|video| html! {
        <VideoDetails video={video.clone()} />
    });

    let videos = vec![
        Video {
            id: 1,
            title: "Building and breaking things".to_string(),
            speaker: "John Doe".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 2,
            title: "The development process".to_string(),
            speaker: "Jane Smith".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 3,
            title: "The Web 7.0".to_string(),
            speaker: "Matt Miller".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 4,
            title: "Mouseless development".to_string(),
            speaker: "Tom Jerry".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
    ];

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideosList videos={videos} on_click={on_video_select.clone()} />
            </div>
            <div>
            { for details }
            </div>
        </>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
