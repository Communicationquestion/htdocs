use component::atoms::text::CssStyle;
use stylist::Style;
use yew::prelude::*;
mod component;
mod css;
mod tools;
use crate::component::atoms::function::RoundCastChart;
use crate::component::atoms::text::PText;
use crate::component::atoms::top::Toplink;
const STYLE_FILE: &str = include_str!("css/divcss/div.css");

#[function_component(App)]

pub fn app() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    html! {
        <div class={stylesheet} style="width:100%;height:100%">
                <div class="header">
                        <Toplink link="" title="首页"/>
                        <Toplink link="" title="博客"/>
                        <Toplink link="" title="社区"/>
                        <Toplink link="" title="Task"/>
                        <Toplink link="" title="Game"/>
                        <Toplink link="" title="About"/>
                </div>

                <div class="main">
                    <RoundCastChart/>
                </div>

                <div class="footer">
                 <PText title="啥也不会的咸鱼" cssstyle={CssStyle::_Bottom}/>
                </div>
        </div>
    }
}
