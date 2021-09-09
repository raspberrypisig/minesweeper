use yew::prelude::*;
use yew::services::ConsoleService;

enum Msg {
    CellClicked(u16),
}

struct Model<T> {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Model<bool>>,
    value: i64,
    leds: [T; 3072]
}

impl Component for Model<bool> {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            leds: [false; 3072]
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CellClicked(x) => {
                self.value += 1;
                let y = x as usize;
                self.leds[y] = !self.leds[y];
                ConsoleService::log(&x.to_string());
                ConsoleService::log(":");
                if self.leds[y] == false {
                    ConsoleService::log("false");
                }
                else {
                    ConsoleService::log("true"); 
                }
                
                
                // the value has changed so we need to
                // re-render for it to appear on the page
                false
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
        let hiddencells = (0..256).map(|i| self.view_hidden_checkbox(i));
        let cells = (0..256).map(|i| self.view_minesweeper_cell(i));
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

impl Model<bool> {
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
    yew::start_app::<Model<bool>>();
}