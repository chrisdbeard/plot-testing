use ndarray::{linspace, Array2, Ix2};
use plotly::{layout::Axis, HeatMap, Layout, Plot, Scatter, Surface};
use serde_json::Value;
use tauri::command;

#[command]
fn generate_plot_json() -> Value {
    let x = vec![1, 2, 3, 4, 5];
    let y = vec![10, 15, 7, 20, 5];

    let trace = Scatter::new(x, y);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let json_value: Value = serde_json::from_str(&plot.to_json()).unwrap();
    json_value
}

#[command]
fn generate_surface_plot_json() -> Value {
    let (x_values, y_values, z_values) = generate_plot_data();

    let trace = Surface::new(z_values)
        .x(x_values)
        .y(y_values)
        .name("3D Surface Plot");
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let scene = plotly::layout::LayoutScene::new()
        .x_axis(Axis::new().title("Time (s)"))
        .y_axis(Axis::new().title("Azimuth Angle (°)"))
        .z_axis(Axis::new().title("Signal Strength (dB)"));

    let layout = Layout::new()
        .title("3D Surface Plot")
        .plot_background_color("grey")
        .paper_background_color("grey")
        .font(plotly::common::Font::new().color("white"))
        .scene(scene)
        .x_axis(
            Axis::new()
                .title("Time (s)")
                .tick_color("white")
                .grid_color("lightgrey"),
        )
        .y_axis(
            Axis::new()
                .title("Azimuth Angle (°)")
                .tick_color("white")
                .grid_color("lightgrey"),
        )
        .z_axis(
            Axis::new()
                .title("Signal Strength (dB)")
                .tick_color("white")
                .grid_color("lightgrey"),
        );

    plot.set_layout(layout);

    let json_value: Value = serde_json::from_str(&plot.to_json()).unwrap();
    json_value
}

#[command]
fn generate_heatmap_plot_json() -> Value {
    let (x_values, y_values, z_values) = generate_plot_data();

    let trace = HeatMap::new(x_values, y_values, z_values);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let layout = Layout::new()
        .title("Heatmap Plot")
        .x_axis(Axis::new().title("Time (s)"))
        .y_axis(Axis::new().title("Azimuth Angle (°)"))
        .z_axis(Axis::new().title("Signal Stregnth (dB)"));

    plot.set_layout(layout);

    let json_value: Value = serde_json::from_str(&plot.to_json()).unwrap();
    json_value
}

fn generate_plot_data() -> (Vec<f64>, Vec<f64>, Vec<Vec<f64>>) {
    let time_steps = 100;
    let angle_steps = 50;

    let time: Vec<f64> = linspace(0.0, 600.0, time_steps).collect();
    let angles: Vec<f64> = linspace(-180.0, 180.0, angle_steps).collect();

    let mut signal_strength = Array2::<f64>::zeros((angle_steps, time_steps));

    let mut signal_mut: ndarray::ArrayViewMut<f64, Ix2> = signal_strength.view_mut();

    for ((i, j), elem) in signal_mut.indexed_iter_mut() {
        let angle = angles[i];
        let t: f64 = time[j];
        let value = 70.0
            + 10.0 * (angle.to_radians() / 90.0).sin() * (t / 300.0).cos()
            + 5.0 * rand::random::<f64>();
        *elem = value;
    }

    let z_values: Vec<Vec<f64>> = signal_strength
        .axis_iter(ndarray::Axis(0))
        .map(|row| row.to_vec())
        .collect();

    let x_values: Vec<f64> = time.clone();
    let y_values: Vec<f64> = angles.clone();

    (x_values, y_values, z_values)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            generate_plot_json,
            generate_surface_plot_json,
            generate_heatmap_plot_json
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_plot_json_structure() {
        let plot_json = generate_plot_json();

        // Check that it's an Object
        assert!(plot_json.is_object(), "Expected a JSON object");

        // Check that "data" field exists
        let data = plot_json
            .get("data")
            .expect("Expected 'data' field in plot JSON");
        assert!(data.is_array(), "'data' field should be an array");

        let traces = data.as_array().unwrap();
        assert_eq!(traces.len(), 1, "Expected exactly one trace");

        // Check the first trace fields
        let trace = &traces[0];
        assert!(trace.get("x").is_some(), "Trace should have an 'x' field");
        assert!(trace.get("y").is_some(), "Trace should have a 'y' field");
        assert_eq!(
            trace.get("type").unwrap().as_str().unwrap(),
            "scatter",
            "Trace type should be 'scatter'"
        );

        // Check x and y values
        let x_values = trace.get("x").unwrap().as_array().unwrap();
        let y_values = trace.get("y").unwrap().as_array().unwrap();

        assert_eq!(
            x_values
                .iter()
                .map(|v| v.as_i64().unwrap())
                .collect::<Vec<_>>(),
            vec![1, 2, 3, 4, 5],
            "X values do not match expected"
        );

        assert_eq!(
            y_values
                .iter()
                .map(|v| v.as_i64().unwrap())
                .collect::<Vec<_>>(),
            vec![10, 15, 7, 20, 5],
            "Y values do not match expected"
        );
    }
}
