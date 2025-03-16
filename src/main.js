const { invoke } = window.__TAURI__.core;

async function loadPlot1() {
  try {
    invoke("generate_plot_json").then((plotJson) => {
      Plotly.newPlot("plot1", plotJson.data, plotJson.layout);
    });
  } catch (error) {
    alert("Failed to generate plot: " + error);
  }
}

// document.getElementById("loadPlot").addEventListener("click", loadPlot);
// window.addEventListener("DOMContentLoaded", loadPlot1);

async function loadPlot2() {
  try {
    invoke("generate_surface_plot_json").then((plotJson) => {
      Plotly.newPlot("plot2", plotJson.data, plotJson.layout);
    });
  } catch (error) {
    alert("Failed to generate plot: " + error);
  }
}

async function loadPlot3() {
  try {
    invoke("generate_heatmap_plot_json").then((plotJson) => {
      Plotly.newPlot("plot3", plotJson.data, plotJson.layout);
    });
  } catch (error) {
    alert("Failed to generate plot: " + error);
  }
}

async function loadPlots() {
  loadPlot1();
  loadPlot2();
  loadPlot3();
}

// document.getElementById("loadPlot").addEventListener("click", loadPlot);
window.addEventListener("DOMContentLoaded", loadPlots);