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
use wasm_bindgen::closure::Closure;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew_chart::{
    axis::{Axis, Orientation, Scale},
    linear_axis_scale::LinearScale,
    series::{BarType, Labeller, Series, Type},
};
// use yew::use_effect_with_deps;
use gloo::utils::document;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use web_sys::console::log_1;

#[derive(Properties, PartialEq)]
pub struct CanvasChartProps {
    pub data: Vec<(f32, f32)>,
    pub chart_type: String,
}

#[function_component(CanvasChart)]
pub fn canvas_chart(props: &CanvasChartProps) -> Html {
    let data = props.data.clone();
    let chart_type = props.chart_type.clone();

    use_effect(move || {
        let canvas = document()
            .get_element_by_id("plotters-canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
        let drawing_area = backend.into_drawing_area();
        drawing_area.fill(&WHITE).unwrap();

        // let y_range = data.iter().map(|(_, y)| *y);
        // let y_min = y_range.clone().fold(f32::MAX, f32::min);
        // let y_max = y_range.clone().fold(f32::MIN, f32::max);
        // let y_max = if (y_max - y_min).abs() < f32::EPSILON { y_max + 1.0 } else { y_max };

        let mut y_min = 0.0;
        let actual_min = data.iter().map(|(_, y)| *y).fold(f32::MAX, f32::min);
        if actual_min < 0.0 {
            y_min = actual_min;
        }

        let mut y_max = data.iter().map(|(_, y)| *y).fold(f32::MIN, f32::max);

        if (y_max - y_min).abs() < f32::EPSILON {
            y_max += 1.0;
            y_min -= 1.0;
        }

        let mut chart = ChartBuilder::on(&drawing_area)
            .caption("Spreadsheet Chart", ("sans-serif", 30).into_font())
            .margin(20)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(0f32..(data.len() as f32), y_min..y_max)
            .unwrap();

        // for zero reference lines

        // chart
        //     .draw_series(LineSeries::new(
        //         vec![(0.0, 0.0), (data.len() as f32, 0.0)],
        //         &BLACK,
        //     ))
        //     .unwrap();

        chart
            .configure_mesh()
            .x_desc("Row Index")
            .y_desc("Value")
            .draw()
            .unwrap();

        // for drawing the line at bottom
        chart
            .draw_series(LineSeries::new(
                vec![(0.0, 0.0), (data.len() as f32, 0.0)],
                &BLACK,
            ))
            .unwrap();

        match chart_type.as_str() {
            "line" => {
                chart
                    .draw_series(LineSeries::new(data.clone(), &RED))
                    .unwrap();
            }
            "bar" => {
                chart
                    .draw_series(data.iter().map(|(x, y)| {
                        let (start, end) = if *y >= 0.0 {
                            ((*x, 0.0), (*x + 0.8, *y))
                        } else {
                            ((*x, *y), (*x + 0.8, 0.0))
                        };
                        Rectangle::new([start, end], RED.filled())
                    }))
                    .unwrap();
            }

            _ => {}
        }

        || ()
    });

    html! {
        <canvas id="plotters-canvas" width="800" height="500" style="border: 1px solid #ccc;" />
    }
}

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

fn number_to_rgb(n: u32) -> (u8, u8, u8) {
    let r = (n >> 16) & 0xFF; 
    let g = (n >> 8) & 0xFF;  
    let b = n & 0xFF;         
    (r as u8, g as u8, b as u8)
}

#[derive(Clone, PartialEq)]
struct SelectedCell {
    row: usize,
    col: usize,
}

#[derive(Clone, PartialEq)]
struct CellRange {
    start: SelectedCell,
    end: SelectedCell,
}

#[function_component(App)]
pub fn app() -> Html {
    let formula_input_ref = use_node_ref();
    // let backend = Backend::init_backend(30, 182);
    // let table = backend.get_valgrid();

    // let table: Valgrid = {
    //     let path = std::env::current_dir()
    //         .map(|p| p.join("grid.json"))
    //         .unwrap_or_else(|_| "grid.json".into());
    //     match fs::read_to_string(path) {
    //         Ok(json) => match serde_json::from_str(&json) {
    //             Ok(grid) => grid,
    //             Err(e) => {
    //                 web_sys::console::error_1(&format!("Failed to parse grid.json: {}", e).into());
    //                 // Return a default grid as fallback
    //                 Valgrid {
    //                     cells: vec![vec![0; 20]; 20],
    //                     rows: 20,
    //                     columns: 20
    //                 }
    //             }
    //         },
    //         Err(e) => {
    //             web_sys::console::error_1(&format!("Failed to read grid.json: {}", e).into());
    //             // Return a default grid as fallback
    //             Valgrid {
    //                 cells: vec![vec![0; 20]; 20],
    //                 rows: 20,
    //                 columns: 20
    //             }
    //         }
    //     }
    // };

    // initialize backend table here

    let max_rows: usize = option_env!("MY_ROWS").unwrap_or("100").parse().unwrap();
    let max_cols: usize = option_env!("MY_COLS").unwrap_or("100").parse().unwrap();
    
    let load_from_json: bool = option_env!("LOAD").map(|v| v == "1" || v.eq_ignore_ascii_case("true")).unwrap_or(false);
    let backend = use_mut_ref(|| Backend::init_backend(max_rows, max_cols)); // debug i dont know the desired dimensions
    // let table = backend.borrow().get_valgrid();

    if load_from_json{
        const CONTEXT: &str = include_str!("../../mysheet.json");
        web_sys::console::log_1(&format!("Context: {}", CONTEXT).into());

        if let Ok(deserialized) = Backend::deserial_text(CONTEXT.to_string()) {
            let backend = backend.clone();
            *backend.borrow_mut() = deserialized;
        } else {
            web_sys::console::error_1(&"Failed to deserialize backend from context".into());
        }
    }

    let table = use_state(|| backend.borrow().get_valgrid());
    let is_formula_building = use_state(|| false);
    // let update_table = {
    //     let backend = backend.clone();
    //     let table = table.clone();
    //     Callback::from(move |_| {
    //         table.set(backend.borrow().get_valgrid());
    //     })
    // };

    let rows1 = use_state(|| 1usize);
    let rows2 = use_state(|| 5usize);
    let cols1 = use_state(|| 1usize);
    let cols2 = use_state(|| 5usize);
    let row_range = *rows1..=(*rows2).min(table.rows - 1);
    let col_range = *cols1..=(*cols2).min(table.columns - 1);
    let selected_cell = use_state(|| None::<SelectedCell>);
    let selected_range = use_state(|| None::<CellRange>);
    let click_anchor = use_state(|| None::<SelectedCell>);
    let selected_column_for_chart = use_state(|| None::<usize>);
    let chart_type = use_state(|| "line".to_string());
    let show_full_table = use_state(|| false);
    let was_inside_table = use_mut_ref(|| false);

    let was_inside_table_click = was_inside_table.clone(); // for on_cell_click
    let was_inside_table_effect = was_inside_table.clone(); // for use_effect

    let selected_range_label = {
        if let Some(range) = &*selected_range {
            let start = &range.start;
            let end = &range.end;
            let label = format!(
                "{}{}:{}{}",
                number_to_column_label(start.col),
                start.row,
                number_to_column_label(end.col),
                end.row
            );
            Some(label)
        } else {
            None
        }
    };

    // i am not calling backend here because it needs a target cell to function
    let selected_range_stats = if let Some(range) = &*selected_range {
        let (start_row, end_row) = if range.start.row <= range.end.row {
            (range.start.row, range.end.row)
        } else {
            (range.end.row, range.start.row)
        };
        let (start_col, end_col) = if range.start.col <= range.end.col {
            (range.start.col, range.end.col)
        } else {
            (range.end.col, range.start.col)
        };
    
        let mut values = vec![];
    
        for r in start_row..=end_row {
            for c in start_col..=end_col {
                if let Some(val) = table.cells.get(r).and_then(|row| row.get(c)) {
                    values.push(*val);
                }
            }
        }
    
        if !values.is_empty() {
            let sum: isize = values.iter().sum();
            let min = *values.iter().min().unwrap_or(&0);
            let max = *values.iter().max().unwrap_or(&0);
            let avg = sum as f64 / values.len() as f64;
            let stdev = {
                let mean = avg;
                let variance: f64 = values
                    .iter()
                    .map(|v| {
                        let diff = *v as f64 - mean;
                        diff * diff
                    })
                    .sum::<f64>()
                    / values.len() as f64;
                variance.sqrt()
            };
    
            Some((sum, min, max, avg, stdev))
        } else {
            None
        }
    } else {
        None
    };
    
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
                let status = backend_ref.process_command(100_usize, 100_usize, command.clone());
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
                        status_message.set(("⚠️ Unrecognized command").to_string());
                    }
                    _ => {
                        // Optional: silently ignore other statuses
                        status_message.set("ℹ️ No update performed.".to_string());
                    }
                }
                // 🟢 now update the table right here:
                // debug
                // table.set(backend_ref.get_valgrid());

                let updated_table = backend_ref.get_valgrid();

                // TEMP DEBUG LOG
                let row_idx = cell.row;
                let col_idx = cell.col;
                if row_idx < updated_table.cells.len()
                    && col_idx < updated_table.cells[row_idx].len()
                {
                    let val = updated_table.cells[row_idx][col_idx];
                    web_sys::console::log_1(
                        &format!("DEBUG: cell[{}, {}] = {}", row_idx, col_idx, val).into(),
                    );
                } else {
                    web_sys::console::log_1(&"DEBUG: selected cell out of bounds".into());
                }

                table.set(updated_table);

                // ✅ reset formula input and mode
                formula_input.set("".to_string());
                is_formula_building.set(false);
            }
        })
    };

    let on_undo = {
        let backend = backend.clone();
        let table = table.clone();
        let status_message = status_message.clone();
    
        Callback::from(move |_| {
            let mut backend_ref = backend.borrow_mut();
            let status = backend_ref.process_command(100, 100, "undo".to_string());
            if let crate::backend::backend::Status::Success = status {
                table.set(backend_ref.get_valgrid());
                status_message.set("↩️ Undo successful".to_string());
            } else {
                status_message.set("⚠️ Nothing to undo".to_string());
            }
        })
    };
    
    let on_redo = {
        let backend = backend.clone();
        let table = table.clone();
        let status_message = status_message.clone();
    
        Callback::from(move |_| {
            let mut backend_ref = backend.borrow_mut();
            let status = backend_ref.process_command(100, 100, "redo".to_string());
            if let crate::backend::backend::Status::Success = status {
                table.set(backend_ref.get_valgrid());
                status_message.set("↪️ Redo successful".to_string());
            } else {
                status_message.set("⚠️ Nothing to redo".to_string());
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
        let was_inside_table = was_inside_table_click.clone(); // capture clone
        let formula_input = formula_input.clone();
        let selected_cell = selected_cell.clone();
        let selected_range = selected_range.clone();
        let click_anchor = click_anchor.clone();
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
    
                    let new_pos = (start + label.len()) as u32;
                    input.set_selection_start(Some(new_pos)).ok();
                    input.set_selection_end(Some(new_pos)).ok();
                    input.focus().ok();
                }
            } else {
                match click_anchor.as_ref() {
                    None => {
                        was_inside_table.borrow_mut().clone_from(&true);
                        selected_cell.set(Some(cell.clone()));
                        selected_range.set(None);
                        click_anchor.set(Some(cell));
                    }
                    Some(anchor) => {
                        was_inside_table.borrow_mut().clone_from(&true);
                        selected_cell.set(Some(cell.clone()));
                        selected_range.set(Some(CellRange {
                            start: anchor.clone(),
                            end: cell,
                        }));
                        click_anchor.set(None);
                    }
                }
            }
        })
    };
    

    #[allow(clippy::type_complexity)]
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
                    let val = row.get(col).copied().unwrap_or(0) as f32;
                    (i as f32, val, None) // Ensure positive values
                })
                .collect();

            // Handle edge case
            if values.is_empty() {
                values = vec![(0.0, 1.0, None)];
            }
            let debug_values: Vec<(f32, f32)> = values.iter().map(|(x, y, _)| (*x, *y)).collect();
            web_sys::console::log_1(&format!("chart data = {:?}", debug_values).into());

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

    fn is_cell_in_range(row: usize, col: usize, range: &CellRange) -> bool {
        let (start_row, end_row) = if range.start.row <= range.end.row {
            (range.start.row, range.end.row)
        } else {
            (range.end.row, range.start.row)
        };
        let (start_col, end_col) = if range.start.col <= range.end.col {
            (range.start.col, range.end.col)
        } else {
            (range.end.col, range.start.col)
        };
        row >= start_row && row <= end_row && col >= start_col && col <= end_col
    }

    {
        let selected_cell = selected_cell.clone();
        let selected_range = selected_range.clone();
        let click_anchor = click_anchor.clone();
    
        use_effect(move || {
            let was_inside_table = was_inside_table_effect.clone(); // use this clone
            let closure = Closure::<dyn FnMut(_)>::wrap(Box::new(move |event: web_sys::MouseEvent| {
                if let Some(target) = event.target() {
                    let tag = target.dyn_ref::<web_sys::Element>().map(|e| e.tag_name());
                    let tag_name = tag.as_deref().unwrap_or("");
        
                    let is_table_cell = tag_name == "TD";
                    let is_input = tag_name == "INPUT";
                    let is_button = tag_name == "BUTTON";
        
                    let clicked_inside = is_table_cell || is_input || is_button;
        
                    web_sys::console::log_1(&format!("tag = {}, clicked_inside = {}", tag_name, clicked_inside).into());
        
                    if !clicked_inside && !*was_inside_table.borrow() {
                        selected_cell.set(None);
                        selected_range.set(None);
                        click_anchor.set(None);
                    }
        
                    *was_inside_table.borrow_mut() = false;
                }
            }) as Box<dyn FnMut(_)>);
        
            let window = web_sys::window().unwrap();
            window
                .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
                .unwrap();
        
            let closure_ref = closure.as_ref().clone();
            let boxed = Box::new(closure);
            move || {
                window
                    .remove_event_listener_with_callback("mousedown", closure_ref.unchecked_ref())
                    .unwrap();
                drop(boxed);
            }
        });        
    }
    
    
    
    html! {
        <div>
            // <div>
            //     <h2>{"Chart Section (using plotters)"}</h2>
            //     <CanvasChart data={data.iter().map(|(x, y, _)| (*x, *y)).collect()} chart_type={(*chart_type).clone()} />
            // </div>
            <style>
            {"
                .selected {
                    background-color: #ffeeba;
                    border: 2px solid #ff9900;
                }
                .range-selected {
                    background-color: #d0f0fd;
                    border: 1px solid #00aaff;
                }
                .highlight-column {
                    background-color: pink;
                }
                .status-bar p {
                    font-weight: bold;
                    margin: 10px;
                    color: #333;
                }
                .table-container {
                    margin: 20px 0;
                    padding: 10px;
                    border-radius: 4px;
                    background: white;
                }
                .table-container h3 {
                    color: #333;
                    margin-bottom: 15px;
                }
                table {
                    width: 100%;
                    border-collapse: collapse;
                    margin-bottom: 20px;
                }
                th, td {
                    border: 1px solid #ddd;
                    padding: 8px;
                    text-align: center;
                }
                th {
                    background-color: #f5f5f5;
                }
                /* Specific styles for the complete table */
                .complete-table {
                    border: none;
                    table-layout: fixed;
                }
                .complete-table td {
                    border: none;
                    padding: 0;
                    width: 20px;
                    height: 20px;
                    min-width: 20px;
                    min-height: 20px;
                }
                .complete-table tr:hover {
                    background: none;
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
                <button style="margin-right: 10px;" onclick={on_submit_formula}>{"Apply"}</button>
                <button style="margin-right: 10px;" onclick={on_undo}>{"Undo"}</button>
                <button onclick={on_redo}>{"Redo"}</button>

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
                <div>
                    <button onclick={
                        let show_full_table = show_full_table.clone();
                        Callback::from(move |_| {
                            show_full_table.set(!*show_full_table);
                        })
                    }>
                        {if *show_full_table { "Hide Image" } else { "Generate Image" }}
                    </button>
                </div>
            </div>

            {if let Some(col) = *selected_column_for_chart {
                let data = get_column_data(col);

                html! {
                    <div>
                        <h2>{ format!("Chart for Column {}", number_to_column_label(col)) }</h2>
                        <CanvasChart
                            data={data.iter().map(|(x, y, _)| (*x, *y)).collect::<Vec<(f32, f32)>>()}
                            chart_type={(*chart_type).clone()}
                        />

                    </div>
                }
            } else {
                html! {}
            }}

            { if let Some(label) = &selected_range_label {
                html! {
                    <div style="margin: 8px 0; font-weight: bold;">
                        { format!("📌 Selected Range: {}", label) }
                        <br/>
                        {
                            if let Some((sum, min, max, avg, stdev)) = &selected_range_stats {
                                html! {
                                    <div style="font-weight: normal; margin-top: 4px;">
                                        { format!("➕ Sum = {sum}, 🟢 Min = {min}, 🔴 Max = {max}, 📊 Avg = {:.2}, 🧮 Stdev = {:.2}", avg, stdev) }
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                }
            } else {
                html! {}
            } }

            <div class="table-container">
                <table>
                    <thead>
                        <tr>
                            <th></th>
                            { for (*cols1..=(*cols2).min(table.columns - 1)).map(|column| {
                                let onclick = {
                                    let on_chart_column_select = on_chart_column_select.clone();
                                    Callback::from(move |_| on_chart_column_select.emit(column))
                                };
                                let is_col_selected = Some(column) == *selected_column_for_chart;
                                html! {
                                    <th
                                        onclick={onclick}
                                        class={if is_col_selected { "highlight-column" } else { "" }}
                                    >
                                        { number_to_column_label(column) }
                                    </th>
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

                                        let in_range = selected_range.as_ref()
                                            .map(|range| is_cell_in_range(row, col, range))
                                            .unwrap_or(false);

                                        let is_in_selected_col = Some(col) == *selected_column_for_chart;

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
                                                class={
                                                    if is_selected {
                                                        "selected"
                                                    } else if in_range {
                                                        "range-selected"
                                                    } else if is_in_selected_col {
                                                        "highlight-column"
                                                    } else {
                                                        ""
                                                    }
                                                }
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
            
            {if *show_full_table {
                html! {
                    <div class="table-container">
                        <h3>{"Image View"}</h3>
                        <table class="complete-table">
                            <tbody>
                                { for (1..=table.rows).map(|row| {
                                    html! {
                                        <tr>
                                            { for (1..=table.columns).map(|col| {
                                                let cell_value = table.cells
                                                    .get(row)
                                                    .and_then(|r| r.get(col))
                                                    .map(|v| v.to_string())
                                                    .unwrap_or_else(|| "".to_string());

                                                let bg_color = if let Ok(num) = cell_value.parse::<u32>() {
                                                    let (r, g, b) = number_to_rgb(num);
                                                    format!("rgb({}, {}, {})", r, g, b)
                                                } else {
                                                    "white".to_string()
                                                };

                                                html! {
                                                    <td style={format!("background-color: {}", bg_color)}>
                                                        { " " }
                                                    </td>
                                                }
                                            }) }
                                        </tr>
                                    }
                                }) }
                            </tbody>
                        </table>
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}

#[allow(dead_code)]
pub fn start_web_app() {
    println!("Starting web PP----------------------------------------------");
    yew::Renderer::<App>::new().render();
}
