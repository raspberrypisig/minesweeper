use yew::prelude::*;
use yew::services::ConsoleService;

enum Msg {
    CellClicked(u16),
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CellClicked(x) => {
                self.value += 1;
                ConsoleService::log(&x.to_string());
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let hiddencells = (1..257).map(|i| self.view_hidden_checkbox(i));
        let cells = (1..257).map(|i| self.view_minesweeper_cell(i));
        html! {
            <div>
             { for hiddencells }
            
             <div id="board" class="grid">
               { for cells }
             </div>
             </div>
        }
}
}

impl Model {
    fn view_hidden_checkbox(&self, idx: u16) -> Html{

    let maybe_id = Some(format!("c{}", idx));
    let name_id = Some(format!("c{}", idx));

     html!{
        <input type="checkbox" id=maybe_id  name=name_id onclick=self.link.callback(move |_| Msg::CellClicked(idx)) />
     }
    }

    fn view_minesweeper_cell(&self, idx: usize) -> Html {
      
      let maybe_id = Some(format!("s{}", idx));
      let for_id = Some(format!("c{}", idx));

      html!{
        <label id=maybe_id class="grid__item" for=for_id></label>
      }
    }
}

fn main() {
    yew::start_app::<Model>();
}