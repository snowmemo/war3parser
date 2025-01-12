import { expect } from "jsr:@std/expect";
import * as path from "jsr:@std/path";
import { Extractor } from "../dist/war3parser.js";

const test_map_path = path.join(Deno.cwd(), "misc", "(6)BlizzardTD.w3x");
const test_map_data = Deno.readFileSync(test_map_path);

Deno.test("extract all", () => {
  const extractor = Extractor.new(new Uint8Array(test_map_data));
  const extract_file = extractor.extract_with_filename("war3map.w3i");
  expect(extract_file?.filename).toBe("war3map.w3i");

  const map_info = extractor.map_info();
  expect(map_info).toBeDefined();

  console.log(map_info?.w3i?.map_name);
  console.log(map_info?.w3i?.version.version_number);
  console.log(map_info?.minimap?.width);
  console.log(map_info?.minimap?.height);
  console.log(map_info?.preview?.width);
  console.log(map_info?.preview?.height);
});
