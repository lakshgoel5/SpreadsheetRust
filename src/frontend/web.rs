use crate::backend::backend::Backend;
#[allow(unused_imports)]
use crate::backend::backend::Valgrid;
#[allow(unused_imports)]
use serde_json;
#[allow(unused_imports)]
use std::fs;
use std::ops::Range;
use std::rc::Rc;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew_chart::{
    axis::{Axis, Orientation, Scale},
    linear_axis_scale::LinearScale,
    series::{BarType, Labeller, Series, Type},
};

/// Converts a numeric column index to an Excel-style column label.
///
/// # Arguments
///
/// * `num` - The column number (1-based index) to convert
///
/// # Returns
///
/// A String representation of the column (e.g., 1 -> "A", 27 -> "AA")
/// Returns an empty string if num is 0
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

/// Represents a selected cell in the spreadsheet grid.
///
/// Contains the row and column indices of the currently selected cell.
#[derive(Clone, PartialEq)]
struct SelectedCell {
    row: usize,
    col: usize,
}

/// Main component for the web-based spreadsheet application.
///
/// Manages the state and user interactions for the web UI, including
/// formula input, cell selection, range viewing, and charting capabilities.
#[function_component(App)]
pub fn app() -> Html {
    let formula_input_ref = use_node_ref();

    // initialize backend table here

    let backend = use_mut_ref(|| Backend::init_backend(100, 100)); // debug i dont know the desired dimensions
    // let table = backend.borrow().get_valgrid();

    let table = use_state(|| backend.borrow().get_valgrid());
    let is_formula_building = use_state(|| false);
    // let update_table = {
    //     let backend = backend.clone();
    //     let table = table.clone();
    //     Callback::from(move |_| {
    //         table.set(backend.borrow().get_valgrid());
    //     })
    // };

    let _max_rows = table.rows; // will change for checking bounds
    let _max_cols = table.columns;

    let rows1 = use_state(|| 1usize);
    let rows2 = use_state(|| 20usize);
    let cols1 = use_state(|| 1usize);
    let cols2 = use_state(|| 20usize);
    let row_range = *rows1..=(*rows2).min(table.rows - 1);
    let col_range = *cols1..=(*cols2).min(table.columns - 1);
    let selected_cell = use_state(|| None::<SelectedCell>);
    let selected_column_for_chart = use_state(|| None::<usize>);
    let chart_type = use_state(|| "bar".to_string());

    let status_message = use_state(|| "".to_string());
    let formula_input = use_state(|| "".to_string());
    let on_formula_input = {
        let formula_input = formula_input.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            formula_input.set(input.value());
        })
    };

    let on_submit_formula = {
        let selected_cell = selected_cell.clone();
        let formula_input = formula_input.clone();
        let backend = backend.clone();
        let table = table.clone(); // 🟢 <- add this
        let status_message = status_message.clone(); // 👈 new

        let is_formula_building = is_formula_building.clone();
        Callback::from(move |_| {
            if let Some(cell) = &*selected_cell {
                let col_label = number_to_column_label(cell.col);
                let row_number = (cell.row).to_string();
                let target_cell = format!("{}{}", col_label, row_number);
                let formula = (*formula_input).clone();
                let command = format!("{}={}", target_cell, formula);

                let mut backend_ref = backend.borrow_mut();
                //web_sys::console::log_1(&format!("Selected cell row={}, col={} => {}", cell.row, cell.col, target_cell).into());
                // web_sys::console::log_1(&format!("Command sent to process_command: {}", command).into());
                let status =
                    backend_ref.process_command(100 as usize, 100 as usize, command.clone());
                match status {
                    crate::backend::backend::Status::Success => {
                        status_message.set(format!("✅ {} updated successfully", target_cell));
                        table.set(backend_ref.get_valgrid());
                    }
                    crate::backend::backend::Status::CircularDependency => {
                        status_message
                            .set(format!("❌ Cycle detected in formula for {}", target_cell));
                    }
                    crate::backend::backend::Status::InvalidRange => {
                        status_message.set(format!("⚠️ Invalid range in formula '{}'", formula));
                    }
                    crate::backend::backend::Status::InvalidRowColumn => {
                        status_message.set(format!("⚠️ Invalid cell reference in '{}'", formula));
                    }
                    crate::backend::backend::Status::UnrecognizedCmd => {
                        status_message.set(format!("⚠️ Unrecognized command"));
                    }
                    _ => {
                        // Optional: silently ignore other statuses
                        status_message.set("ℹ️ No update performed.".to_string());
                    }
                }
                // 🟢 now update the table right here:
                table.set(backend_ref.get_valgrid());

                // ✅ reset formula input and mode
                formula_input.set("".to_string());
                is_formula_building.set(false);
            }
        })
    };

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

    let on_cell_click = {
        let formula_input = formula_input.clone();
        let selected_cell = selected_cell.clone();
        let is_formula_building = is_formula_building.clone();
        let input_ref = formula_input_ref.clone();

        Callback::from(move |cell: SelectedCell| {
            if *is_formula_building {
                let label = format!("{}{}", number_to_column_label(cell.col), cell.row);

                if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                    let mut current = (*formula_input).clone();
                    let start = input
                        .selection_start()
                        .unwrap_or(None)
                        .unwrap_or(current.len() as u32) as usize;
                    let end = input
                        .selection_end()
                        .unwrap_or(None)
                        .unwrap_or(current.len() as u32) as usize;

                    current.replace_range(start..end, &label);
                    formula_input.set(current);

                    // move cursor after inserted text
                    let new_pos = (start + label.len()) as u32;
                    input.set_selection_start(Some(new_pos)).ok();
                    input.set_selection_end(Some(new_pos)).ok();
                    input.focus().ok();
                }
            } else {
                selected_cell.set(Some(cell));
            }
        })
    };

    let get_column_data = {
        let table = table.clone();
        move |col: usize| -> Vec<(f32, f32, Option<Rc<dyn Labeller>>)> {
            // Collect and normalize data
            let mut values: Vec<(f32, f32, Option<Rc<dyn Labeller>>)> = table
                .cells
                .iter()
                .enumerate()
                .take(20)
                .map(|(i, row)| {
                    let val = row[col - 1] as f32;
                    (i as f32, val.max(0.1), None) // Ensure positive values
                })
                .collect();

            // Handle edge case
            if values.is_empty() {
                values = vec![(0.0, 1.0, None)];
            }

            values
        }
    };

    let on_chart_column_select = {
        let selected_column_for_chart = selected_column_for_chart.clone();
        Callback::from(move |col: usize| {
            selected_column_for_chart.set(Some(col));
        })
    };

    let on_chart_type_change = {
        let chart_type = chart_type.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<HtmlSelectElement>();
            chart_type.set(input.value());
        })
    };

    html! {
        <div>
            <style>
            {"
                .selected {
                    background-color: #ffeeba;
                    border: 2px solid #ff9900;
                }
                .status-bar p {
                    font-weight: bold;
                    margin: 10px;
                    color: #333;
                }
            "}
            </style>
            <div class="formula-bar">
                <input
                    ref={formula_input_ref.clone()}
                    type="text"
                    value={(*formula_input).clone()}
                    oninput={on_formula_input}
                    onfocus={Callback::from({
                        let is_formula_building = is_formula_building.clone();
                        let input_ref = formula_input_ref.clone();
                        move |_| {
                            is_formula_building.set(true);
                            if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                                input.focus().ok(); // Reinforce focus
                            }
                        }
                    })}
                    // onblur={Callback::from({
                    //     let is_formula_building = is_formula_building.clone();
                    //     move |_| is_formula_building.set(false)
                    // })}
                    placeholder="Enter formula eg. SUM(B1:B10)"
                />
                <button onclick={on_submit_formula}>{"Apply"}</button>
            </div>
            <div class="status-bar">
                <p>{ (*status_message).clone() }</p>
            </div>
            <div class="controls">
                <div>
                    <label>{"Rows: "}</label>
                    <input type="number" value={(*rows1).to_string()} oninput={on_rows1_change} min="1" max="100"/>
                    {" to "}
                    <input type="number" value={(*rows2).to_string()} oninput={on_rows2_change} min="1" max="100"/>
                </div>
                <div>
                    <label>{"Columns: "}</label>
                    <input type="number" value={(*cols1).to_string()} oninput={on_cols1_change} min="1" max="100"/>
                    {" to "}
                    <input type="number" value={(*cols2).to_string()} oninput={on_cols2_change} min="1" max="100"/>
                </div>
                <div>
                    <label>{"Chart Type: "}</label>
                    <select value={(*chart_type).clone()} onchange={on_chart_type_change}>
                        <option value="bar">{"Bar"}</option>
                        <option value="line">{"Line"}</option>
                    </select>
                </div>
            </div>

            {if let Some(col) = *selected_column_for_chart {
                let data = get_column_data(col);
                let y_max = data.iter().map(|(_, y, _)| *y).fold(0.0f32, |a, b| a.max(b));
                let x_max = data.len() as f32;

                // SVG dimensions
                let width = 800.0;
                let height = 500.0;
                let margin = 60.0;
                let chart_width = width - 2.0 * margin;
                let chart_height = height - 2.0 * margin;

                let x_scale = Rc::new(LinearScale::new(
                    Range { start: 0.0, end: x_max },
                    1.0
                )) as Rc<dyn Scale<Scalar = f32>>;

                let y_scale = Rc::new(LinearScale::new(
                    Range { start: 0.0, end: y_max * 1.1 },
                    (y_max / 5.0).max(1.0)
                )) as Rc<dyn Scale<Scalar = f32>>;

                html! {
                    <div class="chart-container">
                        <svg
                            width={width.to_string()}
                            height={height.to_string()}
                            style="background: white;"
                        >
                            // Y-axis (left side)
                            <g transform={format!("translate({},{})", margin, margin)}>
                                <Axis<f32>
                                    name="y-axis"
                                    orientation={Orientation::Left}
                                    scale={y_scale.clone()}
                                    tick_len={5.0}
                                    x1={0.0}
                                    y1={0.0}
                                    xy2={chart_height}
                                    title="Value"
                                />
                            </g>

                            // X-axis (bottom)
                            <g transform={format!("translate({},{})", margin, height - margin)}>
                                <Axis<f32>
                                    name="x-axis"
                                    orientation={Orientation::Bottom}
                                    scale={x_scale.clone()}
                                    tick_len={5.0}
                                    x1={0.0}
                                    y1={0.0}
                                    xy2={chart_width}
                                    title="Row"
                                />
                            </g>

                            // Chart series
                            <g transform={format!("translate({},{})", margin, height - margin)}>
                                <g transform="scale(1,-1)">
                                    <Series<f32, f32>
                                        data={Rc::new(data)}
                                        height={chart_height}
                                        width={chart_width}
                                        x={0.0}
                                        y={0.0}
                                        horizontal_scale={x_scale}
                                        vertical_scale={y_scale}
                                        name={format!("Column {}", number_to_column_label(col))}
                                        series_type={if (*chart_type).as_str() == "bar" { Type::Bar(BarType::Rise) } else { Type::Line }}
                                    />
                                </g>
                            </g>
                        </svg>
                    </div>
                }
            } else {
                html! {}
            }}

            <div class="table-container">
                <table>
                    <thead>
                        <tr>
                            <th></th>
                            { for (*cols1..=*cols2).map(|col| {
                                let onclick = {
                                    let on_chart_column_select = on_chart_column_select.clone();
                                    let col = col.clone();
                                    Callback::from(move |_| on_chart_column_select.emit(col))
                                };
                                html! {
                                    <th onclick={onclick}>{ number_to_column_label(col) }</th>
                                }
                            }) }
                        </tr>
                    </thead>
                    <tbody>
                        { for row_range.clone().map(|row| {
                            html! {
                                <tr>
                                    <th>{ row }</th>
                                    { for col_range.clone().map(|col| {
                                        // let cell_value = table.cells[row - 1][col - 1].to_string();
                                        let cell_value = table.cells
                                            .get(row)
                                            .and_then(|r| r.get(col))
                                            .map(|v| v.to_string())
                                            .unwrap_or_else(|| "ERR".to_string());
                                        let is_selected = selected_cell.as_ref()
                                            .map(|sc| sc.row == row && sc.col == col)
                                            .unwrap_or(false);
                                        let onclick = {
                                            let on_cell_click = on_cell_click.clone();
                                            let row = row;
                                            let col = col;
                                            Callback::from(move |_| {
                                                on_cell_click.emit(SelectedCell { row, col });
                                            })
                                        };
                                        html! {
                                            <td
                                                onclick={onclick}
                                                class={if is_selected { "selected" } else { "" }}
                                            >
                                                { cell_value }
                                            </td>
                                        }
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

/// Initializes and starts the web application.
///
/// Creates a new Yew renderer for the App component and renders it
/// to the DOM. This function is the entry point for the web interface.
#[allow(dead_code)]
pub fn start_web_app() {
    println!("Starting web PP----------------------------------------------");
    yew::Renderer::<App>::new().render();
}
