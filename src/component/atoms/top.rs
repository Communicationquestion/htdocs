use stylist::{style};
use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub link: String,
}

#[function_component(Toplink)]
pub fn tolink(props: &Props) -> Html {
    let _stylesheet = style!(
        r#"
            width: 16%;
            height: 88%;
            float: right;     /*div浮动 重叠的移动到右边*/
            margin-left: 100px;
            margin-right: 100px;
            
            display: table;
            overflow: auto;
           
            .font:hover{
                background-color: aqua;
                opacity: 1;
                transition-duration: 3s;
            }
            .font:active{
                color:aqua;
            }
            .font{
                display:table-cell;
                vertical-align:middle;
                text-align:center;  /*  文字居中*/
                text-decoration:none;
                border-radius: 25px;
                
            }
        "#
    ).unwrap();
    
    html!(
        <div class={_stylesheet}>   /*接受样式 在下面标签里设置*/
            <a href={props.link.to_string()} class="font" >{&props.title}</a> 
        </div>
    )
}
