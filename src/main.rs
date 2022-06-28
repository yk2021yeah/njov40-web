use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();

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

        let videos = videos
            .iter()
            .map(|video| {
                html! {
                    <p>{format!("{}: {}, {}", video.speaker, video.title, video.url)}</p>
                }
            })
            .collect::<Html>();

        html! {
            <div class="panel">
                <div class="left_pane">
                    <table>
                        <tr>
                            <td class="left_pain">
                                <h1>{ "RustConf Explorer" }</h1>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <button class="button" onclick={link.callback(|_| Msg::AddOne)}>{ "Rust" } </button>
                                <button class="button" onclick={link.callback(|_| Msg::AddOne)}>{ "Rust" } </button>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <p>{ self.value }</p>
                            </td>
                        </tr>
                    </table>
                </div>
                <div class="right_pane">
                    <table class="right_pane">
                        <h3>{"Videos to watch"}</h3>
                        { videos }
                        <h3>{ "John Doe: Building and breaking things" }</h3>
                        <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder"
                            alt="video thumbnail" />
                    </table>
                </div>

            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
