use yew::prelude::*;
use yew::services::ConsoleService;

const MATRIX_WIDTH: usize = 16;
const MATRIX_HEIGHT: usize = 16;
const PANEL_WIDTH: usize = 6;
const PANEL_HEIGHT: usize = 2;
const LED_COUNT: usize = MATRIX_WIDTH * MATRIX_HEIGHT * PANEL_WIDTH * PANEL_HEIGHT;

enum Msg {
    CellClicked(u16),
    SwitchSegment(usize),
}

struct Model<T> {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Model<bool>>,
    value: i64,
    leds: [T; LED_COUNT],
    panel_horizontal_offset: usize,
    panel_vertical_offset: usize,
    current_segment: usize,
}

impl Component for Model<bool> {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            leds: [false; LED_COUNT],
            panel_horizontal_offset: 0,
            panel_vertical_offset: 0,
            current_segment: 1,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CellClicked(x) => {
                self.value += 1;
                let y = x as usize;
                self.leds[y] = !self.leds[y];
                ConsoleService::log(&x.to_string());               
                if self.leds[y] == false {
                    ConsoleService::log("false");
                }
                else {
                    ConsoleService::log("true"); 
                }
                let finalx = x as usize % MATRIX_WIDTH + MATRIX_WIDTH * self.panel_horizontal_offset * MATRIX_WIDTH;
                let finaly = x as usize / MATRIX_WIDTH + self.panel_vertical_offset * MATRIX_HEIGHT;
                ConsoleService::log(&finalx.to_string());
                ConsoleService::log(&finaly.to_string());
                //ConsoleService::log(&self.panel_vertical_offset.to_string());                
                //ConsoleService::log(&LED_COUNT.to_string());
                // the value has changed so we need to
                // re-render for it to appear on the page
                false
            }

            Msg::SwitchSegment(x) => {
                self.current_segment = x;
                self.panel_horizontal_offset = (x - 1) % PANEL_WIDTH;
                self.panel_vertical_offset = (x - 1) / PANEL_WIDTH;
                ConsoleService::log(&x.to_string());
                ConsoleService::log(&self.panel_horizontal_offset.to_string());
                ConsoleService::log(&self.panel_vertical_offset.to_string());

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
        let hiddencells = (0..256).map(|i| self.view_hidden_checkbox(i));
        let cells = (0..256).map(|i| self.view_minesweeper_cell(i));
        let buttons_top = (1..7).map(|i| self.section_buttons(i));
        let buttons_bottom = (7..13).map(|i| self.section_buttons(i));
        html! {
            <div>
             { for hiddencells }
            
             <div id="board" class="grid">
               { for cells }
             </div>

            <div class="buttons">
              {for buttons_top}
            </div>

            <div class="buttons">
              {for buttons_bottom}
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

    fn selected_button(&self, idx: usize) -> Option<String>{
        if idx == self.current_segment {
            Some("selected".to_string())
        }
        else {
        None
        }

       
    }
 
    fn section_buttons(&self, idx: usize) -> Html {
        let buttonid = Some(format!("b{}", idx));
        html!{
            <button id=buttonid class=self.selected_button(idx) onclick=self.link.callback(move |_| Msg::SwitchSegment(idx) ) >{idx.to_string()}</button>
        }
    }
}

fn main() {
    yew::start_app::<Model<bool>>();
}