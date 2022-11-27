
use wasm_bindgen;
use yew::{function_component, html};

#[function_component(RoundCastChart)]

pub fn round_cast_chart() -> Html {

    let stylesheet = style!(
        r#"
            position: relative;            
            width: 1000px;
            height: 500px;
            margin:20px auto 0 auto;
            box-shadow: 0 0 5px green;
            overflow: hidden;
            .wrap{
                overflow: hidden;
                position: absolute;
                width: 4200px;
                height: 500px;
                z-index: 1;

            }
            .wrap img{
                float: left;
                width: 1000px;
                height: 500px;
            }

            .buttons{
                position: absolute;
                left: 0;
                right: 0;
                margin: auto;
                bottom:20px;
                width: 300px;
                height: 20px;
                z-index: 2;
              
               
            }

            .buttons span {
               
                margin-left: 50px;
                display: inline-block;
                width: 10px;
                height: 10px;
                border-radius: 50%;
                
               
                background-color: aqua;
                opacity: 0.3;
                cursor: pointer;
            }
        
        "#
    )
    .unwrap();
    //let string_style="left:-3000px;";
   
    let g_src_array = ["left:0;", "left:-1000px;", "left:-2000px;", "left:-3000px;"];
    
   
  
    let counter=use_state(||0);

   
    html!(
        <div class={stylesheet}>

            <div class="wrap"  style={g_src_array[*counter]}>

                <img src="../../../static/1.jpg" alt=""/>
                <img src="../../../static/2.jpg" alt=""/>
                <img src="../../../static/3.jpg" alt=""/>
                <img src="../../../static/4.jpg" alt=""/>
            </div>

            <div class="buttons">
                <span></span>
                <span></span>
                <span></span>
                <span></span>
                <span></span>
            </div>

        </div>
    )
}
