use wasm_bindgen::prelude::*;
use web_sys::{
    console,Document, Element, Window
};
use js_sys::Function;

//This macro is used to log messages to the browser console
macro_rules! log {
    ($($t:tt)*) => 
        (console::log_1(&format!($($t)*).into()));
}

//This here is the entry point for the WASM module
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue>{
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    //Starts the auto-swipe loop
    start_auto_swipe(&window, &document)?;
    Ok(())
}

//Function to start the auto-swipe loop
fn start_auto_swipe(window: &Window, document: &Document)-> Result<(), JsValue>{
    //This is the interval in milliseconds (1000ms = 1second)
    let interval_ms = 1000.0;

    //Closure to handle the auto-swipe logic

    let document = document.clone();
    let callback = Closure::wrap(Box::new(move ||{
        if let Err(e) = auto_swipe(&document) {
            log!("Error during auto-swipe: {:?}", e);
        }
    }) as Box<dyn FnMut()>);


    // Set up a setInterval to run the callback every second
    window.set_interval_with_callback_and_timeout_and_arguments_0(
        callback.as_ref().unchecked_ref::<Function>(),
        interval_ms as i32,
    )?;

    //Keeps the closure alive for the lifetime of the program
    callback.forget();
    Ok(())
}

//Thingy to perform the main auto swipe logic
fn auto_swipe(document: &Document) -> Result<(), JsValue>{
    //Swipes the monsters
    let monsters = document.query_selector_all(".monster")?;
    for i in 0..monsters.length() {
        let monster_node = monsters.item(i).unwrap();
        let monster = monster_node.dyn_ref::<Element>().ok_or_else(|| JsValue::from_str("Monster is not an Element"))?;
        click_element(monster)?;
        log!("Tagged monster {}", i);
    }

    //Click upgrades
    let upgrades = document.query_selector_all(".upgrade")?;
    for i in 0..upgrades.length() {
        let upgrade_node = upgrades.item(i).unwrap();
        let upgrade = upgrade_node.dyn_ref::<Element>().ok_or_else(|| JsValue::from_str("Upgrade is not an Element"))?;

        //Check if the upgrade is clickable
        if !upgrade.has_attribute("disabled") {
            click_element(upgrade)?;
            log!("Clicked upgrade #{}", i);
        }
    }
    Ok(())
}

//Helper function

fn click_element(element: &Element)-> Result<(), JsValue>{
    element.dyn_ref::<web_sys::HtmlElement>()
        .ok_or_else(|| JsValue::from_str("Element is not an HtmlElement"))?
        .click();
    Ok(())
}

//M1