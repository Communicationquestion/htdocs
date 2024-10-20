use gloo_timers::callback::Timeout;
use stylist::style;
use yew::{function_component, html, use_state};

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
    let counter: yew::UseStateHandle<usize> = use_state(|| 0);
    let counter_x = counter.clone();

    let timeout = Timeout::new(1_000, move || {
        // Do something after the one second timeout is up!
        let counter = counter.clone();
        if *counter > 2 {
            counter.set(0); // resets the counter to 0 after the first 3 times it is called.
        } else {
            counter.set(*counter + 1);
        }
    });

    timeout.forget();

    html!(
        <div class={stylesheet}>

            <div class="wrap"  style={g_src_array[*counter_x]}>

                <img src="../../../static/1.jpg" alt=""/>
                <img src="../../../static/2.jpg" alt=""/>
                <img src="../../../static/3.jpg" alt=""/>
                <img src="../../../static/4.jpg" alt=""/>
            </div>

            <div class="buttons">
                <span ></span>
                <span ></span>
                <span ></span>
                <span ></span>

            </div>

        </div>
    )
}
