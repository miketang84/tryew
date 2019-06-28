use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::ConsoleService;

pub struct MyApp {
	console: ConsoleService,
}

pub enum Msg {
    Click,
}

impl Component for MyApp {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        MyApp {
			console: ConsoleService::new(),
		}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
				self.console.log("clicked")
            }
        }
        true
    }
}

impl Renderable<MyApp> for MyApp {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <button onclick=|_| Msg::Click,>{ "Click" }</button>
            </div>
        }
    }
}

