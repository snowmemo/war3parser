import {describe, expect, test} from '@jest/globals';
import * as w from '../dist/node';
import * as path from 'path';
import { readFileSync } from 'fs';

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
        if (w3i === undefined) throw new Error("w3i is undefined");
        expect(w3i).toBeDefined();
        expect(w3i.filename()).toBe("war3map.w3i");
    })

    test("extract with file type", () => {
        const map = readFileSync(MAP_PATH);
        const extractor = w.create_extractor(map);
        const w3i = extractor.extract(w.War3Format.W3i);
        if (w3i === undefined) throw new Error("w3i is undefined");
        expect(w3i).toBeDefined();
        expect(w3i.filename()).toBe("war3map.w3i");
    })
});