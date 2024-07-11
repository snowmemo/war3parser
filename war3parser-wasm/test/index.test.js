const w = require("../dist/node");
const path = require('path');

const { readFileSync, writeFileSync } = require("fs");

const MAP_PATH = path.join(__dirname, '..', '..', 'misc', '(6)BlizzardTD.w3x');

describe("extractor", () => {
    test("create extract", () => {
        const map = readFileSync(MAP_PATH);
        const extractor = w.create_extractor(map);
        expect(extractor).toBeDefined();
    })

    test("extract with filename", () => {
        const map = readFileSync(MAP_PATH);
        const extractor = w.create_extractor(map);
        const w3i = extractor.extract_with_filename("war3map.w3i");
        expect(w3i).toBeDefined();
        expect(w3i.filename()).toBe("war3map.w3i");
    })

    test("extract with file type", () => {
        const map = readFileSync(MAP_PATH);
        const extractor = w.create_extractor(map);
        const w3i_type = new w.War3Format('w3i');
        const w3i = extractor.extract(w3i_type);
        expect(w3i).toBeDefined();
        expect(w3i.filename()).toBe("war3map.w3i");
    })
});