use crate::backend::backend;
use yew::prelude::*;

// Add this function before the App component
fn number_to_column_label(num: usize) -> String {
    if num == 0 {
        return String::new();
    }

    let mut result = String::new();
    let mut n = num;

    while n > 0 {
        n -= 1;
        let c = ((n % 26) as u8 + b'A') as char;
        result.insert(0, c);
        n /= 26;
    }

    result
}

#[function_component(App)]
pub fn app() -> Html {
    let backend = backend::Backend::init_backend(30, 182);
    let table = backend.get_valgrid();
    let rows1 = use_state(|| 1usize);
    let rows2 = use_state(|| 20usize);
    let cols1 = use_state(|| 1usize);
    let cols2 = use_state(|| 20usize);

    let on_rows1_change = {
        let rows1 = rows1.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            if let Ok(value) = input.value().parse::<usize>() {
                rows1.set(value);
            }
        })
    };

    let on_rows2_change = {
        let rows2 = rows2.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            if let Ok(value) = input.value().parse::<usize>() {
                rows2.set(value);
            }
        })
    };

    let on_cols1_change = {
        let cols1 = cols1.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            if let Ok(value) = input.value().parse::<usize>() {
                cols1.set(value);
            }
        })
    };

    let on_cols2_change = {
        let cols2 = cols2.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            if let Ok(value) = input.value().parse::<usize>() {
                cols2.set(value);
            }
        })
    };

    html! {
        <div>
            <div class="controls">
                <label>
                    { "Rows start: " }
                    <input type="number" value={rows1.to_string()} oninput={on_rows1_change} min="1" max="999" />
                </label>
                <label>
                    { " Rows end: " }
                    <input type="number" value={rows2.to_string()} oninput={on_rows2_change} min="1" max="999" />
                </label>
                <label>
                    { " Cols start: " }
                    <input type="number" value={cols1.to_string()} oninput={on_cols1_change} min="1" max="999" />
                </label>
                <label>
                    { " Cols end: " }
                    <input type="number" value={cols2.to_string()} oninput={on_cols2_change} min="1" max="999" />
                </label>
            </div>

            <div class="table-container">
                <table>
                    <thead>
                        <tr>
                            <th></th> // Empty corner cell
                            { for (*cols1..=*cols2).map(|col| html! {
                                <th>{ number_to_column_label(col) }</th>
                            }) }
                        </tr>
                    </thead>
                    <tbody>
                        { for (*rows1..=*rows2).map(|row| {
                            html! {
                                <tr>
                                    <th>{ row }</th>
                                    { for (*cols1..=*cols2).map(|col| html! {
                                        <td>{ table.cells[row-1][col-1] }</td>
                                    }) }
                                </tr>
                            }
                        }) }
                    </tbody>
                </table>
            </div>
        </div>
    }
}

pub fn start_web_app() {
    yew::Renderer::<App>::new().render();
}

#[allow(dead_code)]
fn main() {
    start_web_app();
}
