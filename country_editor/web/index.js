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

window.onload = () => {
    /*
    let grid = [[], [], []];
    addLawSelectorToColumn("lawSelector1", {"agrarianism": "../resources/agrarianism.png", "interventionism": "../resources/interventionism.png"}, 0, grid)
    // wait 5 seconds
    setTimeout(() => {
        clearGrid(grid);
        addLawSelectorToColumn("lawSelector2", {"agrarianism": "../resources/agrarianism.png", "interventionism": "../resources/interventionism.png"}, 1, grid)
    }, 3000);
    */
}