// src/frontend/app.rs
mod parser;

#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[function_component(App)]
pub fn app() -> Html {
    // State management
    let rows = use_state(|| 10usize);  // Default rows
    let cols = use_state(|| 10usize);  // Default columns
    let command = use_state(String::new);
    let output = use_state(String::new);
    let error = use_state(String::new);

    // Input handlers
    let on_rows_change = {
        let rows = rows.clone();
        let error = error.clone();
        Callback::from(move|e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            if let Ok(value) = input.value().parse::<usize>() {
                rows.set(value);
                error.set(String::new());
            } else {
                error.set("Invalid number of rows".into());
            }
        })
    };

    let on_cols_change = {
        let cols = cols.clone();
        let error = error.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            if let Ok(value) = input.value().parse::<usize>() {
                cols.set(value);
                error.set(String::new());
            } else {
                error.set("Invalid number of columns".into());
            }
        })
    };

    let on_command_submit = {
        let rows = rows.clone();
        let cols = cols.clone();
        let command = command.clone();
        let output = output.clone();
        let error = error.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            // Validate grid dimensions
            if *rows > 999 || *cols > 18278 {
                error.set("Rows must be ≤999 and columns ≤18278 (ZZZ)".into());
                return;
            }

            // Validate command
            match parser::validate(&command, &rows, &cols) {
                Some(cell) => {
                    output.set(format!("Valid cell: {:?}", cell));
                    error.set(String::new());
                }
                None => {
                    error.set("Invalid command".into());
                    output.set(String::new());
                }
            }
        })
    };

    html! {
        <div class="spreadsheet-gui">
            <h1>{ "Rust Spreadsheet" }</h1>
            
            // Grid configuration
            <div class="config">
                <label>
                    { "Rows: " }
                    <input
                        type="number"
                        value={rows.to_string()}
                        oninput={on_rows_change}
                        min="1"
                        max="999"
                    />
                </label>
                
                <label>
                    { "Columns: " }
                    <input
                        type="number"
                        value={cols.to_string()}
                        oninput={on_cols_change}
                        min="1"
                        max="18278"
                    />
                </label>
            </div>

            // Command input
            <form onsubmit={on_command_submit}>
                <input
                    type="text"
                    placeholder="Enter command (e.g. A2=3+4)"
                    value={(*command).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                        command.set(input.value());
                    })}
                />
                <button type="submit">{ "Validate" }</button>
            </form>

            // Display output/errors
            {if !error.is_empty() {
                html! { <div class="error">{ &*error }</div> }
            } else {
                html! {}
            }}
            
            {if !output.is_empty() {
                html! { <div class="output">{ &*output }</div> }
            } else {
                html! {}
            }}
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
fn start_web_app() {
    yew::Renderer::<App>::new().render();
}

// #[cfg(target_arch = "wasm32")]
// #[function_component]
//     fn App() -> Html {
//         let counter = use_state(|| 0);
//         let onclick = {
//             let counter = counter.clone();
//             move |_| {
//                 let value = *counter + 1;
//                 counter.set(value);
//             }
//         };

//         html! {
//             <div>
//                 <button {onclick}> {"+1"} </button>
//                 <p>{*counter}</p>
//             </div>
//         }
//     }

#[cfg(not(target_arch = "wasm32"))]
use std::env;

#[cfg(not(target_arch = "wasm32"))]
fn start_terminal_app() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <rows> <columns>", args[0]);
        std::process::exit(1);
    }
    let rows: usize = args[1].parse().expect("Invalid number of rows");
    let columns: usize = args[2].parse().expect("Invalid number of columns");
    if rows>999 || columns>18278 {
        eprintln! ("Invalid input: rows and cols need to be within 999 and ZZZ respectively");
        std::process::exit(1);
    }

    // reading command and replacing the trailing newline with null character
    let mut cmd = String::new();
    let _bytes_read = std::io::stdin().read_line(&mut cmd).expect("Failed to read command");
    let cmd = String::from(cmd.trim());

    // calling parser
    let cell = parser::validate(&cmd, &rows, &columns);
    if let Some(c) = cell {
        println!("{:?}", c);
    } else {
        eprintln!("Invalid command");
    }  
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    start_web_app();

    #[cfg(not(target_arch = "wasm32"))]
    start_terminal_app();
}