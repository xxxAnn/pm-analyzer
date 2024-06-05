
// Read the contents of the file
const filePath = 'data/country_data.tsv';

async function load_data() {
    return await fetch(filePath)
        .then(response => response.text())
        .then(text => {
            // Split the contents into lines
            const lines = text.split('\n');

            // Create an empty object to store the mapping
            const iso3ToNameShort = {};

            // Iterate over each line and extract the ISO3 and name_short values
            for (let line of lines) {
                let splitLine = line.split('\t');
                iso3ToNameShort[splitLine[4]] = splitLine[0];
            }

            return iso3ToNameShort;
        });
}