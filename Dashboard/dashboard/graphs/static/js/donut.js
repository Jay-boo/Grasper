let myPieChart = null;
Chart.defaults.font.family = '"Poppins", sans-serif';
Chart.defaults.elements.arc.borderWidth = 0;
Chart.defaults.elements.arc.hoverOffset = 15;
// Function to aggregate data by theme
function aggregateDataByTheme(data) {
    const themeCounts = {};

    data.forEach(item => {
        item.tags.forEach(tagItem => {
            const theme = tagItem.tag.theme;
            if (theme in themeCounts) {
                themeCounts[theme]++;
            } else {
                themeCounts[theme] = 1;
            }
        });
    });

    return themeCounts;
}



// Function to aggregate data by labels within a theme
function aggregateDataByLabels(data, theme) {
    const labelCounts = {};

    data.forEach(item => {
        item.tags.forEach(tagItem => {
            if (tagItem.tag.theme === theme) {
                console.log(tagItem.tag.theme);
                const label = tagItem.tag.label;
                if (label in labelCounts) {
                    labelCounts[label]++;
                } else {
                    labelCounts[label] = 1;
                }
            }
        });
    });

    return labelCounts;
}

function calculatePercentages(values) {
    const total = values.reduce((sum, value) => sum + value, 0);
    const percentages = values.map(value => (value / total) * 100);
    return percentages;
}
// Function to render the pie chart using Chart.js
function renderPieChart(data, theme = null) {
    let aggregatedData, chartTitle;

    if (theme) {
        aggregatedData = aggregateDataByLabels(data, theme);
        chartTitle = `Distribution des labels dans le thème "${theme}"`;
    } else {
        aggregatedData = aggregateDataByTheme(data);
        chartTitle = 'Distribution des thèmes';
    }

    const labels = Object.keys(aggregatedData);
    const values = Object.values(aggregatedData);
    const values_percent = calculatePercentages(values);
    const ctx = document.getElementById('myPieChart').getContext('2d');

    if (myPieChart) {
        myPieChart.destroy();
    }

    myPieChart = new Chart(ctx, {
        type: 'doughnut',
        data: {
            datasets: [{
                data: values,
                backgroundColor: ['#845ec2', '#2c73d2', '#0081cf', '#0089ba', '#008e9b', '#008f7a'],
                datalabels: {
                  labels: {
                    name: {
                      align: 'top',
                      font: {size: 14},
                      formatter: function(value, ctx) {
                        return labels[ctx.dataIndex];
                      }
                    },
                    value: {
                      align: 'bottom',
                      borderColor: 'white',
                      borderWidth: 2,
                      borderRadius: 4,
                      color: 'white',
                      formatter: function(value, ctx) {
                        const value_to_print = values_percent[ctx.dataIndex]
                        return Math.round(value_to_print * 100) / 100 + " %";
                      },
                      padding: 4,
                      offset: -5 
                    }
                  }
                }
            }]
        },
        options: {
            responsive: true,
            animation: {
              animateScale: true
            },
            plugins: {
                datalabels: {
                    color: '#fff',
                    formatter: function(value, context) {
                        const datapoints = context.chart.data.datasets[0].data
                        const total = datapoints.reduce((total, datapoint) => total + datapoint, 0)
                        const percentage = value / total * 100
                        return labels[context.dataIndex];
                    },
                    font: {
                        weight: 'normal',
                        size: 16
                    }
                },
                title: {
                    display: true,
                    text: chartTitle,
                    color: '#000000',
                    font: {
                        size: 18,
                        weight: 'normal'
                    }
                },
                legend: {
                  display: false
                }
            },
            onClick: (event, elements) => {
                if (elements.length > 0) {
                    const chartElement = elements[0];
                    const labelIndex = chartElement.index;
                    console.log(chartElement);
                    console.log(labels);
                    if (!theme) {
                        renderPieChart(data, labels[labelIndex]);
                        document.getElementById('resetButton').style.display = 'block';
                    }
                }
            }
        },
        plugins: [ChartDataLabels]
    });

    return myPieChart;
}

function fetchDataBasedOnSelection(selection) {
    let url = '/graphs/messages_with_tags/';
    if (selection === 'Les 7 derniers jours') {
        url += '?period=last_7_days';
    } else if (selection === 'Le dernier mois') {
        url += '?period=last_month';
    } else if (selection === 'Les 3 derniers mois') {
        url += '?period=last_3_months';
    } else if (selection === 'Les 6 derniers mois') {
        url += '?period=last_6_months';
    }
    fetch(url)
        .then(response => response.json())
        .then(data => {
            renderPieChart(data);
        })
        .catch(error => console.error('Error fetching data:', error));
}

document.getElementById('resetButton').addEventListener('click', () => {
    fetch('/graphs/messages_with_tags/')
        .then(response => response.json())
        .then(data => {
            renderPieChart(data);
            document.getElementById('resetButton').style.display = 'none'; // Hide reset button
        })
        .catch(error => console.error('Error fetching data:', error));
});

document.getElementById('Dropdown').addEventListener('change', (event) => {
    fetchDataBasedOnSelection(event.target.value);
});

// Fetch data from the endpoint and render the chart
fetch('/graphs/messages_with_tags/')
    .then(response => response.json())
    .then(data => {
        renderPieChart(data);
    })
    .catch(error => console.error('Error fetching data:', error));
