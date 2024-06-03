let myLineChart;


function aggregateDataByTimePeriod(data, period, theme_given = null) {
    const themeCountsByPeriod = {};

    data.forEach(item => {
        const date = new Date(item.date);
        let periodKey;
        
        if (period === 'by_weeks') {
            const weekStart = new Date(date);
            weekStart.setDate(date.getDate() - date.getDay() + (date.getDay() === 0 ? -6 : 1)); // Set to Monday
            periodKey = weekStart.toISOString().split('T')[0]; // YYYY-MM-DD format for Monday
        } else if (period === 'by_months') {
            periodKey = date.getFullYear() + '-' + (date.getMonth() + 1).toString().padStart(2, '0');
        } else {
            periodKey = date.toISOString().split('T')[0]; // Daily format YYYY-MM-DD
        }

        item.tags.forEach(tagItem => {

            const theme = theme_given ? tagItem.tag.label : tagItem.tag.theme;

            
            if (!themeCountsByPeriod[periodKey]) {
                themeCountsByPeriod[periodKey] = {};
            }
            if (!themeCountsByPeriod[periodKey][theme]) {
                themeCountsByPeriod[periodKey][theme] = 0;
            }
            themeCountsByPeriod[periodKey][theme] += 1;
        });
    });
    console.log(themeCountsByPeriod);
    return themeCountsByPeriod;
}

function prepareLineChartData(themeCountsByPeriod, period) {
    const periods = Object.keys(themeCountsByPeriod).sort();
    const themes = [...new Set(Object.values(themeCountsByPeriod).flatMap(Object.keys))];

    const datasets = themes.map(theme => {
        return {
            label: theme,
            data: periods.map(period => themeCountsByPeriod[period][theme] || 0),
            fill: false
            
        };
    });

    return { periods, datasets };
}

function renderLineChart(data, period, theme_given=null) {
    const aggregatedData = aggregateDataByTimePeriod(data, period, theme_given);
    const { periods, datasets } = prepareLineChartData(aggregatedData, period);
    const ctx = document.getElementById('myLineChart').getContext('2d');

    if (myLineChart) {
        myLineChart.destroy();
    }

    myLineChart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: periods,
            datasets: datasets
        },
        options: {
            responsive: true,
            title: {
                display: true,
                text: 'Number of Messages per Theme per ' + (period === 'by_weeks' ? 'Week' : period === 'by_months' ? 'Month' : 'Day')
            },
            tension : 0,
            scales: {
                x: {
                    type: 'category',
                    labels: periods,
                    time: {
                        unit: period === 'by_weeks' ? 'week' : period === 'by_months' ? 'month' : 'day',
                        displayFormats: {
                            day: 'MMM D',
                            week: 'MMM D',
                            month: 'MMM YYYY'
                        }
                    },
                    title: {
                        display: true,
                        text: 'Date'
                    }
                },
                y: {
                    beginAtZero: true,
                    title: {
                        display: true,
                        text: 'Count'
                    }
                }
            },
            onClick: (evt) => {
                const points = myLineChart.getElementsAtEventForMode(evt, 'nearest', axis = 'xy', { intersect: true }, true);
                if (points.length && !theme_given) {
                    const firstPoint = points[0];
                    const theme = myLineChart.data.datasets[firstPoint.datasetIndex].label;
                    fetchDetailedDataByTheme(theme, period);
                } else {
                  fetchDataBasedOnSelectionLine(period);
                }
            }
        }
    });

    return myLineChart;
}

function fetchDetailedDataByTheme(theme, period) {
    let url = `/graphs/messages_with_tags/?theme=${encodeURIComponent(theme)}`;
    fetch(url)
        .then(response => response.json())
        .then(data => {
            renderLineChart(data, period, theme);
        })
        .catch(error => console.error('Error fetching detailed data:', error));
}


function fetchDataBasedOnSelectionLine(selection) {
    let url = '/graphs/messages_with_tags/';
    // Add your existing period selection logic if needed
    fetch(url)
        .then(response => response.json())
        .then(data => {
            renderLineChart(data, selection);
        })
        .catch(error => console.error('Error fetching data:', error));
}

document.getElementById('DropdownLine').addEventListener('change', (event) => {
    fetchDataBasedOnSelectionLine(event.target.value);
});

// Fetch data from the endpoint and render the chart initially with default period
fetch('/graphs/messages_with_tags/')
    .then(response => response.json())
    .then(data => {
        renderLineChart(data, 'by_months');
    })
    .catch(error => console.error('Error fetching data:', error));

