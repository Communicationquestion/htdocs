use stylist::style;
use yew::{function_component, html, Properties};
#[derive(PartialEq)]
pub enum CssStyle {
    _Top,
    _Normal,
    _Bottom,
}


impl CssStyle {
   
    pub fn to_string(&self) -> String {
        match self {
            CssStyle::_Top => "top".to_owned(),
            CssStyle::_Normal => "normal".to_owned(),
            CssStyle::_Bottom => "bottom".to_owned(),
        

        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub cssstyle: CssStyle,
}

#[function_component(PText)]
pub fn text(props: &Props) -> Html {
    let stylesheet = style!(
        r#"
           
            text-align:center;  /*  文字居中*/
            width: 100%;
            display: table;
            overflow: auto;
            .top{
                color: green;
            }

            .normal{
                color:white;
            }

            .bottom{
                color: white;
                display:table-cell;
                vertical-align:middle;
                font-size: 1rem;
                text-transform: capitalize;
                text-shadow: 1px 1px 1px red, 2px 2px 1px red;
                text-align: center;
                letter-spacing: 2px;

            }
        "#
    )
    .unwrap();
    html!(
        <div class={stylesheet}>
            <p class={props.cssstyle.to_string()}>{&props.title}</p>
        </div>
    )
}
