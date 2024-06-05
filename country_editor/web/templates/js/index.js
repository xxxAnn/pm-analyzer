const raw = {
    backgroundPosition: '0px 0px',
    backgroundSize: '100% 100%',
    backgroundRepeat: 'no-repeat',
    backgroundAttachment: 'scroll',
    backgroundOrigin: 'padding-box',
}

function applyDictStyle(style, object) {
    for (let [key, value] of Object.entries(style)) {
        object.style[key] = value;
    }
}

function setCustomBackgroundImage(imageUrl, object) {
    let style = {
        width: '20%',
        backgroundImage: `url(${imageUrl})`,
        backgroundPosition: '0px 0px',
        backgroundSize: '100% 100%',
        backgroundRepeat: 'no-repeat',
        backgroundAttachment: 'scroll',
        backgroundOrigin: 'padding-box',
        fontSize: '0px',
        marginTop: '0%',
        marginRight: '40%',
        marginBottom: '10%',
        marginLeft: '40%',
    };

    for (let [key, value] of Object.entries(style)) {
        object.style[key] = value;
    }
}

function newLawSelector(id, options_to_image) {
    let newSelector = document.createElement('select');
    newSelector.type = 'select';
    newSelector.id = id;
    newSelector.className = 'lawSelector';
    for (option of Object.keys(options_to_image)) {
        let opt = document.createElement('option');
        opt.value = option;
        opt.text = toPrettyName(option)
        opt.style.fontSize = '16px';
        applyDictStyle(raw, opt);
        newSelector.appendChild(opt);
    }
    newSelector.addEventListener('change', function() {
        setCustomBackgroundImage(options_to_image[this.value], newSelector);
    });
    setCustomBackgroundImage(options_to_image[newSelector.value], newSelector);

    return newSelector;
}

function addLawSelectorToColumn(id, options_to_image, column, grid) {
    column = column % 3;
    let clmn = document.getElementById(["it2b", "i3dm", "inq9b"][column]);
    let lawSelector = newLawSelector(id, options_to_image);
    clmn.appendChild(lawSelector);
    grid[column].push(lawSelector);
}

function clearGrid(grid) {

    document.getElementById("if4j").innerHTML = `
    <div id="it2b" class="gjs-grid-column">
    </div>
    <div id="i3dm" class="gjs-grid-column">
    </div>
    <div id="inq9b" class="gjs-grid-column">
    </div>
    `
    return [[], [], []];
}

function populateCountryList(countryList) {
    const dropdownList = document.querySelector('.dropdown-list');
    dropdownList.innerHTML = '';

    countryList.forEach(country => {
        const label = document.createElement('label');
        label.innerHTML = `<input type="checkbox" value="${country}"> ${country}`;
        dropdownList.appendChild(label);
    });

    addCheckboxListeners();
}

function populateCountryListPairs(countryList) {
    const dropdownList = document.querySelector('.dropdown-list');
    dropdownList.innerHTML = '';

    for (country of countryList) {
        const label = document.createElement('label');
        label.innerHTML = `<input type="checkbox" value="${country[0]}"> ${country[1]}`;
        dropdownList.appendChild(label);
    }

    addCheckboxListeners();
}

function getSelectedCountries() {
    const selectedCountries = [];
    const checkboxes = document.querySelectorAll('.dropdown-list input:checked');
    checkboxes.forEach(checkbox => {
        selectedCountries.push(checkbox.value);
    });
    return selectedCountries;
}

function addCheckboxListeners() {
    const selectedOptions = document.querySelector('.selected-options');
    document.querySelectorAll('.dropdown-list input').forEach(input => {
        input.addEventListener('change', function() {

            selectedOptions.innerHTML = '';
            const checkedInputs = document.querySelectorAll('.dropdown-list input:checked');
            const maxSpans = 3;
            let spanCount = 0;
            checkedInputs.forEach(checkedInput => {
                if (spanCount < maxSpans) {
                    const span = document.createElement('span');
                    span.textContent = checkedInput.value;
                    selectedOptions.appendChild(span);
                    spanCount++;
                }
            });

            if (checkedInputs.length > maxSpans) {
                const span = document.createElement('span');
                span.textContent = `+ ${checkedInputs.length - maxSpans} others`;
                selectedOptions.appendChild(span);
            }

            const countryDropbox = document.querySelector('.country-dropbox');
            countryDropbox.classList.toggle('active', checkedInputs.length > 0);
        });
    });
}

// Code

const countries = [
    "Afghanistan", "Albania", "Algeria", "American Samoa", "Andorra", "Angola"
];

document.addEventListener('DOMContentLoaded', function() {
    const dropdownList = document.querySelector('.dropdown-list');

    populateCountryList(countries);

    document.querySelector('.country-dropbox').addEventListener('click', function(event) {
        event.stopPropagation();
        dropdownList.classList.toggle('active');
    });

    document.addEventListener('click', function(event) {
        console.log(valueGrid(grid))
        if (!dropdownList.contains(event.target) && !event.target.closest('.country-dropbox')) {
            dropdownList.classList.remove('active');
        }
    });
});

var grid = [[], [], []];

window.onload = () => {
    
    addLawSelectorToColumn("lawSelector1", {"agrarianism": "/resources/agrarianism.png", "interventionism": "/resources/interventionism.png"}, 0, grid)
    load()
}

function valueGrid(grid) {
    let result = [];
    for (let column of grid) {
        let columnResult = [];
        for (let lawSelector of column) {
            columnResult.push(lawSelector.value);
        }
        result.push(columnResult);
    }
    return result;

}

async function load() {
    let default_data = JSON.parse(await fetch("/api/defaultstate").then(response => response.json()));
    let pairs = [];


    grid = clearGrid(grid);

    for (let [lawgroupname, o] of Object.entries(default_data.laws)) {
        console.log(o[0]);
        addLawSelectorToColumn(lawgroupname, o[0], o[1], grid);
    }

    for (let country of default_data.countries) {
        let countryName = await fetch(`/api/countryname/${country}`).then(response => response.json());
        pairs.push([country, (countryName == "N/A") ? country : countryName]);
    }

    document.getElementById("io4cj").style.display = "none";

    populateCountryListPairs(pairs);
}

function capitalize(name) {
    return name.charAt(0).toUpperCase() + name.slice(1);
}
function toPrettyName(lawName) {
    return capitalize(lawName.replace("law_", "").replaceAll("_", " ").toLowerCase());
}