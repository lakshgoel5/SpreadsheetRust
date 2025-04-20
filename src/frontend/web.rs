use yew::prelude::*;
use std::rc::Rc;
use std::ops::Range;
use crate::backend::backend::Backend;
use yew_chart::{
    axis::{Axis, Orientation, Scale},
    linear_axis_scale::LinearScale,
    series::{Series, Labeller, Type, BarType},
};
use web_sys::HtmlSelectElement;

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

#[derive(Clone, PartialEq)]
struct SelectedCell {
    row: usize,
    col: usize,
}

#[function_component(App)]
pub fn app() -> Html {
    let backend = Backend::init_backend(30, 182);
    let table = backend.get_valgrid();
    let rows1 = use_state(|| 1usize);
    let rows2 = use_state(|| 20usize);
    let cols1 = use_state(|| 1usize);
    let cols2 = use_state(|| 20usize);
    let selected_cell = use_state(|| None::<SelectedCell>);
    let selected_column_for_chart = use_state(|| None::<usize>);
    let chart_type = use_state(|| "bar".to_string());

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
        let selected_cell = selected_cell.clone();
        Callback::from(move |cell: SelectedCell| {
            selected_cell.set(Some(cell));
        })
    };

    let get_column_data = {
        let table = table.clone();
        move |col: usize| -> Vec<(f32, f32, Option<Rc<dyn Labeller>>)> {
            // Collect and normalize data
            let mut values: Vec<(f32, f32, Option<Rc<dyn Labeller>>)> = table.cells
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
            <div class="controls">
                <div>
                    <label>{"Rows: "}</label>
                    <input type="number" value={(*rows1).to_string()} oninput={on_rows1_change} min="1" max="30"/>
                    {" to "}
                    <input type="number" value={(*rows2).to_string()} oninput={on_rows2_change} min="1" max="30"/>
                </div>
                <div>
                    <label>{"Columns: "}</label>
                    <input type="number" value={(*cols1).to_string()} oninput={on_cols1_change} min="1" max="182"/>
                    {" to "}
                    <input type="number" value={(*cols2).to_string()} oninput={on_cols2_change} min="1" max="182"/>
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
                        { for (*rows1..=*rows2).map(|row| {
                            html! {
                                <tr>
                                    <th>{ row }</th>
                                    { for (*cols1..=*cols2).map(|col| {
                                        let cell_value = table.cells[row - 1][col - 1].to_string();
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

pub fn start_web_app() {
    yew::Renderer::<App>::new().render();
}