use yew::{prelude::*, services::ConsoleService};
pub enum Msg {
    Increment,
    Decrement,
}
struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value += 1;
                ConsoleService::log("plus one");
                true
            }
            Msg::Decrement => {
                self.value -= 1;
                ConsoleService::log("minus one");
                true
            }
        }
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <div>
                <div>{"Value: "}{self.value}</div>
                <div>
                    <button onclick=self.link.callback(|_| Msg::Increment)>{"Add"}</button>
                    <button onclick=self.link.callback(|_| Msg::Decrement)>{"Remove"}</button>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>()
}
