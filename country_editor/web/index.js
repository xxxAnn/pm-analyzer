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
        opt.text = option.charAt(0).toUpperCase() + option.slice(1);
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
    let clmn = document.getElementById(["it2b", "i3dm", "inq9b"][column]);
    let lawSelector = newLawSelector(id, options_to_image);
    clmn.appendChild(lawSelector);
    grid[column].push(lawSelector);
}

function clearGrid(grid) {
    for (let column of grid) {
        for (let lawSelector of column) {
            lawSelector.remove();
        }
    }
    grid = [[], [], []];
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
            console.log("Hello");

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
        if (!dropdownList.contains(event.target) && !event.target.closest('.country-dropbox')) {
            dropdownList.classList.remove('active');
        }
    });
});

window.onload = () => {
    
    let grid = [[], [], []];
    addLawSelectorToColumn("lawSelector1", {"agrarianism": "../resources/agrarianism.png", "interventionism": "../resources/interventionism.png"}, 0, grid)
    // wait 5 seconds
    // why was that comment there it's clearly 3 seconds
    setTimeout(() => {
        clearGrid(grid);
        addLawSelectorToColumn("lawSelector2", {"agrarianism": "../resources/agrarianism.png", "interventionism": "../resources/interventionism.png"}, 1, grid)
    }, 3000);
    
}