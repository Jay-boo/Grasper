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
            tension: 0,
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

fetch('/graphs/messages_with_tags/')
    .then(response => response.json())
    .then(data => {
        renderLineChart(data, 'by_months');
        setupDateRangeSlider(data);
    })
    .catch(error => console.error('Error fetching data:', error));

function setupDateRangeSlider(data) {
    const uniqueDates = [...new Set(data.map(item => new Date(item.date).toISOString().split('T')[0]))]
        .map(date => new Date(date));
    uniqueDates.sort((a, b) => a - b);

    const dateRangeSlider = document.getElementById('dateRangeSlider');
    noUiSlider.create(dateRangeSlider, {
        start: [0, uniqueDates.length - 1],
        connect: true,
        range: {
            'min': 0,
            'max': uniqueDates.length - 1
        },
        step: 1,
        tooltips: [true, true],
        format: {
            to: value => Math.round(value),
            from: value => Math.round(value)
        }
    });

    document.getElementById('minDateLabel').textContent = uniqueDates[0].toISOString().split('T')[0];
    document.getElementById('maxDateLabel').textContent = uniqueDates[uniqueDates.length - 1].toISOString().split('T')[0];

    dateRangeSlider.noUiSlider.on('update', (values, handle) => {
        const selectedStartDate = uniqueDates[values[0]];
        const selectedEndDate = uniqueDates[values[1]];
        document.getElementById('minDateLabel').textContent = selectedStartDate.toISOString().split('T')[0];
        document.getElementById('maxDateLabel').textContent = selectedEndDate.toISOString().split('T')[0];
        filterDataByDateRange(selectedStartDate, selectedEndDate, data);
    });
}

function filterDataByDateRange(startDate, endDate, data) {
    const filteredData = data.filter(item => {
        const itemDate = new Date(item.date);
        return itemDate >= startDate && itemDate <= endDate;
    });
    renderLineChart(filteredData, document.getElementById('DropdownLine').value);
}
