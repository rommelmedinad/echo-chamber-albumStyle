use gloo_net::http::{Headers, Request};
use web_sys::RequestCredentials;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use crate::router::Route;
use crate::types::Album;

pub enum AlbumMsg {
    SetAlbums(Vec<Album>),
}

pub struct AlbumPageComponent {
    album_components: Vec<AlbumComponent>,
}

impl Component for AlbumPageComponent {
    type Message = AlbumMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            // let fetched_albums = Request::get("").send().await.unwrap().json().await.unwrap();
            let fetched_albums = Request::get("http://127.0.0.1:8000/key/albums")
                .header("Accept", "application/json")
                .header("NS", "echochamber")
                .header("DB", "echochamber")
                .send()
                .await
                .expect("Crashed on GET");
            let fetched = fetched_albums.json().await.expect("Crashed on JSON");
            AlbumMsg::SetAlbums(vec![])
        });
        Self {
            album_components: vec![],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AlbumMsg::SetAlbums(albums) => {
                let album_components = albums
                    .iter()
                    .map(|album| AlbumComponent {
                        cover: album.cover.clone(),
                        title: album.title.clone(),
                        artist: album.album_artist.clone(),
                    })
                    .collect();
                self.album_components = album_components;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let albums = self.album_components.clone();
        let albums = albums.iter().map(|_| {
            html! { <AlbumComponent /> }
        });
        html! {
            <div>
                <h1 class="title">{ "Albums" }</h1>
                <div class="columns is-multiline is-mobile">
                    { for albums }
                </div>
            </div>
        }
    }
}

#[derive(Clone, Properties, PartialEq)]
struct AlbumComponent {
    cover: String,
    title: String,
    artist: String,
}

impl Component for AlbumComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let css_string = format!(
            r#"
            :root {{
                /* Color Vars /
                --background-color: {};
                --primary-color: {};
                --accent-color: {};
                --light-gray: {};
                / Font Vars */
                --font-title: "{}", cursive;
                --font-text: "{}", sans-serif;
            }}
            .card {{
                background-color: var(--background-color);
                color: var(--light-gray);
                font-family: var(--font-text);
            }}
            "#,
            "rgb(52, 58, 64)",
            "rgb(166, 231, 242)",
            "rgb(87, 204, 153)",
            "rgb(72, 78, 84)",
            "Righteous",
            "Fira Sans"
        );
        html! {
            <div class=css!(css_string)>
            <div class="column is-one-third-mobile is-one-quarter-tablet is-one-fifth-desktop">
                <div class="card">
                    <div class="card-image">
                        <figure class="image is-square">
                            <img src={ self.cover.clone() } alt="Album Cover" />
                        </figure>
                    </div>
                    <div class="card-content has-text-centered">
                        <div class="content">
                            <p>{ self.title.clone() }</p>
                            <p>{ self.artist.clone() }</p>
                        </div>
                    </div>
                </div>
            </div>
            </div>
        }
    }
}

impl AlbumComponent {
    fn default() -> Self {
        Self {
            cover: "https://via.placeholder.com/200".into(),
            title: "Album Title".into(),
            artist: "Artist Name".into(),
        }
    }
}
