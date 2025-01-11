import { expect } from "jsr:@std/expect";
import * as path from "jsr:@std/path";
import { instantiate } from "../dist/war3parser.generated.js";
const { Extractor } = await instantiate();

const test_map_path = path.join(Deno.cwd(), "misc", "(6)BlizzardTD.w3x");
const test_map_data = Deno.readFileSync(test_map_path);

Deno.test("extract w3i", () => {
  const extractor = Extractor.new(new Uint8Array(test_map_data));
  const extract_file = extractor.extract_with_filename("war3map.w3i");
  expect(extract_file?.filename).toBe("war3map.w3i");

  const map_info = extractor.map_info();
  expect(map_info).toBeDefined();

  console.log(map_info?.map_name);
});



