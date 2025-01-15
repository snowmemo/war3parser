/* tslint:disable */
/* eslint-disable */
export class WasmForce {
  private constructor();
  free(): void;
  flags: number;
  player_masks: number;
  name: string;
}
export class WasmImage {
  private constructor();
  free(): void;
  width: number;
  height: number;
  data: ImageData;
}
export class WasmMapInfo {
  private constructor();
  free(): void;
  static new(buffer: Uint8Array): WasmMapInfo | undefined;
  get map_info(): WasmW3i | undefined;
  set map_info(value: WasmW3i | null | undefined);
  images: WasmImage[];
}
export class WasmPlayer {
  private constructor();
  free(): void;
  id: number;
  player_type: number;
  race: number;
  is_fixed_start_position: number;
  name: string;
  start_location: Float32Array;
  ally_low_priorities: number;
  ally_high_priorities: number;
  get known1(): Uint8Array | undefined;
  set known1(value: Uint8Array | null | undefined);
}
export class WasmRandomItem {
  private constructor();
  free(): void;
  chance: number;
  id: Uint8Array;
}
export class WasmRandomItemSet {
  private constructor();
  free(): void;
  items: WasmRandomItem[];
}
export class WasmRandomItemTable {
  private constructor();
  free(): void;
  id: number;
  name: string;
  sets: WasmRandomItemSet[];
}
export class WasmRandomUnit {
  private constructor();
  free(): void;
  chance: number;
  ids: Uint8Array[];
}
export class WasmRandomUnitTable {
  private constructor();
  free(): void;
  id: number;
  name: string;
  position: number;
  column_types: Int32Array;
  units: WasmRandomUnit[];
}
export class WasmTechAvailabilityChange {
  private constructor();
  free(): void;
  player_flags: number;
  id: Uint8Array;
}
export class WasmUpgradeAvailabilityChange {
  private constructor();
  free(): void;
  player_flags: number;
  id: Uint8Array;
  level_affected: number;
  availability: number;
}
export class WasmW3i {
  private constructor();
  free(): void;
  version: number;
  saves: number;
  editor_version: number;
  get build_version(): Uint32Array | undefined;
  set build_version(value: Uint32Array | null | undefined);
  name: string;
  author: string;
  description: string;
  recommended_players: string;
  camera_bounds: Float32Array;
  camera_bounds_complements: Uint32Array;
  playable_size: Uint32Array;
  flags: number;
  tileset: number;
  campaign_background: number;
  get loading_screen_model(): string | undefined;
  set loading_screen_model(value: string | null | undefined);
  loading_screen_text: string;
  loading_screen_title: string;
  loading_screen_subtitle: string;
  loading_screen: number;
  get prologue_screen_model(): string | undefined;
  set prologue_screen_model(value: string | null | undefined);
  prologue_screen_text: string;
  prologue_screen_title: string;
  prologue_screen_subtitle: string;
  get use_terrain_fog(): number | undefined;
  set use_terrain_fog(value: number | null | undefined);
  get fog_height(): Float32Array | undefined;
  set fog_height(value: Float32Array | null | undefined);
  get fog_density(): number | undefined;
  set fog_density(value: number | null | undefined);
  get fog_color(): Uint8Array | undefined;
  set fog_color(value: Uint8Array | null | undefined);
  get global_weather(): number | undefined;
  set global_weather(value: number | null | undefined);
  get sound_environment(): string | undefined;
  set sound_environment(value: string | null | undefined);
  get light_environment_tileset(): number | undefined;
  set light_environment_tileset(value: number | null | undefined);
  get water_vertex_color(): Uint8Array | undefined;
  set water_vertex_color(value: Uint8Array | null | undefined);
  get script_mode(): number | undefined;
  set script_mode(value: number | null | undefined);
  get graphics_mode(): number | undefined;
  set graphics_mode(value: number | null | undefined);
  get unknown1(): number | undefined;
  set unknown1(value: number | null | undefined);
  players: WasmPlayer[];
  forces: WasmForce[];
  upgrade_availability_changes: WasmUpgradeAvailabilityChange[];
  tech_availability_changes: WasmTechAvailabilityChange[];
  random_unit_tables: WasmRandomUnitTable[];
  random_item_tables: WasmRandomItemTable[];
}
export class WasmWts {
  private constructor();
  free(): void;
  string_map: Map<any, any>;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_wasmplayer_free: (a: number, b: number) => void;
  readonly __wbg_get_wasmplayer_id: (a: number) => number;
  readonly __wbg_set_wasmplayer_id: (a: number, b: number) => void;
  readonly __wbg_get_wasmplayer_player_type: (a: number) => number;
  readonly __wbg_set_wasmplayer_player_type: (a: number, b: number) => void;
  readonly __wbg_get_wasmplayer_race: (a: number) => number;
  readonly __wbg_set_wasmplayer_race: (a: number, b: number) => void;
  readonly __wbg_get_wasmplayer_is_fixed_start_position: (a: number) => number;
  readonly __wbg_set_wasmplayer_is_fixed_start_position: (a: number, b: number) => void;
  readonly __wbg_get_wasmplayer_name: (a: number) => [number, number];
  readonly __wbg_set_wasmplayer_name: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmplayer_start_location: (a: number) => any;
  readonly __wbg_set_wasmplayer_start_location: (a: number, b: any) => void;
  readonly __wbg_get_wasmplayer_ally_low_priorities: (a: number) => number;
  readonly __wbg_set_wasmplayer_ally_low_priorities: (a: number, b: number) => void;
  readonly __wbg_get_wasmplayer_ally_high_priorities: (a: number) => number;
  readonly __wbg_set_wasmplayer_ally_high_priorities: (a: number, b: number) => void;
  readonly __wbg_get_wasmplayer_known1: (a: number) => any;
  readonly __wbg_set_wasmplayer_known1: (a: number, b: number) => void;
  readonly __wbg_wasmforce_free: (a: number, b: number) => void;
  readonly __wbg_get_wasmforce_flags: (a: number) => number;
  readonly __wbg_set_wasmforce_flags: (a: number, b: number) => void;
  readonly __wbg_get_wasmforce_player_masks: (a: number) => number;
  readonly __wbg_set_wasmforce_player_masks: (a: number, b: number) => void;
  readonly __wbg_get_wasmforce_name: (a: number) => [number, number];
  readonly __wbg_set_wasmforce_name: (a: number, b: number, c: number) => void;
  readonly __wbg_wasmrandomitem_free: (a: number, b: number) => void;
  readonly __wbg_get_wasmrandomitem_chance: (a: number) => number;
  readonly __wbg_set_wasmrandomitem_chance: (a: number, b: number) => void;
  readonly __wbg_get_wasmrandomitem_id: (a: number) => any;
  readonly __wbg_set_wasmrandomitem_id: (a: number, b: any) => void;
  readonly __wbg_wasmrandomitemset_free: (a: number, b: number) => void;
  readonly __wbg_get_wasmrandomitemset_items: (a: number) => [number, number];
  readonly __wbg_set_wasmrandomitemset_items: (a: number, b: number, c: number) => void;
  readonly __wbg_wasmrandomitemtable_free: (a: number, b: number) => void;
  readonly __wbg_get_wasmrandomitemtable_sets: (a: number) => [number, number];
  readonly __wbg_set_wasmrandomitemtable_sets: (a: number, b: number, c: number) => void;
  readonly __wbg_wasmrandomunit_free: (a: number, b: number) => void;
  readonly __wbg_get_wasmrandomunit_ids: (a: number) => [number, number];
  readonly __wbg_set_wasmrandomunit_ids: (a: number, b: number, c: number) => void;
  readonly __wbg_wasmrandomunittable_free: (a: number, b: number) => void;
  readonly __wbg_get_wasmrandomunittable_id: (a: number) => number;
  readonly __wbg_set_wasmrandomunittable_id: (a: number, b: number) => void;
  readonly __wbg_get_wasmrandomunittable_column_types: (a: number) => [number, number];
  readonly __wbg_set_wasmrandomunittable_column_types: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmrandomunittable_units: (a: number) => [number, number];
  readonly __wbg_set_wasmrandomunittable_units: (a: number, b: number, c: number) => void;
  readonly __wbg_wasmupgradeavailabilitychange_free: (a: number, b: number) => void;
  readonly __wbg_get_wasmupgradeavailabilitychange_level_affected: (a: number) => number;
  readonly __wbg_set_wasmupgradeavailabilitychange_level_affected: (a: number, b: number) => void;
  readonly __wbg_wasmw3i_free: (a: number, b: number) => void;
  readonly __wbg_get_wasmw3i_version: (a: number) => number;
  readonly __wbg_set_wasmw3i_version: (a: number, b: number) => void;
  readonly __wbg_get_wasmw3i_saves: (a: number) => number;
  readonly __wbg_set_wasmw3i_saves: (a: number, b: number) => void;
  readonly __wbg_get_wasmw3i_editor_version: (a: number) => number;
  readonly __wbg_set_wasmw3i_editor_version: (a: number, b: number) => void;
  readonly __wbg_get_wasmw3i_name: (a: number) => [number, number];
  readonly __wbg_set_wasmw3i_name: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmw3i_author: (a: number) => [number, number];
  readonly __wbg_set_wasmw3i_author: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmw3i_description: (a: number) => [number, number];
  readonly __wbg_set_wasmw3i_description: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmw3i_recommended_players: (a: number) => [number, number];
  readonly __wbg_set_wasmw3i_recommended_players: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmw3i_camera_bounds: (a: number) => any;
  readonly __wbg_set_wasmw3i_camera_bounds: (a: number, b: any) => void;
  readonly __wbg_get_wasmw3i_camera_bounds_complements: (a: number) => any;
  readonly __wbg_set_wasmw3i_camera_bounds_complements: (a: number, b: any) => void;
  readonly __wbg_get_wasmw3i_playable_size: (a: number) => any;
  readonly __wbg_set_wasmw3i_playable_size: (a: number, b: any) => void;
  readonly __wbg_get_wasmw3i_flags: (a: number) => number;
  readonly __wbg_set_wasmw3i_flags: (a: number, b: number) => void;
  readonly __wbg_get_wasmw3i_tileset: (a: number) => number;
  readonly __wbg_set_wasmw3i_tileset: (a: number, b: number) => void;
  readonly __wbg_get_wasmw3i_campaign_background: (a: number) => number;
  readonly __wbg_set_wasmw3i_campaign_background: (a: number, b: number) => void;
  readonly __wbg_get_wasmw3i_loading_screen_model: (a: number) => [number, number];
  readonly __wbg_set_wasmw3i_loading_screen_model: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmw3i_loading_screen_text: (a: number) => [number, number];
  readonly __wbg_set_wasmw3i_loading_screen_text: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmw3i_loading_screen_title: (a: number) => [number, number];
  readonly __wbg_set_wasmw3i_loading_screen_title: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmw3i_loading_screen_subtitle: (a: number) => [number, number];
  readonly __wbg_set_wasmw3i_loading_screen_subtitle: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmw3i_loading_screen: (a: number) => number;
  readonly __wbg_set_wasmw3i_loading_screen: (a: number, b: number) => void;
  readonly __wbg_get_wasmw3i_prologue_screen_model: (a: number) => [number, number];
  readonly __wbg_set_wasmw3i_prologue_screen_model: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmw3i_prologue_screen_text: (a: number) => [number, number];
  readonly __wbg_set_wasmw3i_prologue_screen_text: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmw3i_prologue_screen_title: (a: number) => [number, number];
  readonly __wbg_set_wasmw3i_prologue_screen_title: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmw3i_prologue_screen_subtitle: (a: number) => [number, number];
  readonly __wbg_set_wasmw3i_prologue_screen_subtitle: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmw3i_use_terrain_fog: (a: number) => number;
  readonly __wbg_set_wasmw3i_use_terrain_fog: (a: number, b: number) => void;
  readonly __wbg_get_wasmw3i_fog_height: (a: number) => any;
  readonly __wbg_set_wasmw3i_fog_height: (a: number, b: number) => void;
  readonly __wbg_get_wasmw3i_fog_density: (a: number) => number;
  readonly __wbg_set_wasmw3i_fog_density: (a: number, b: number) => void;
  readonly __wbg_get_wasmw3i_fog_color: (a: number) => any;
  readonly __wbg_set_wasmw3i_fog_color: (a: number, b: number) => void;
  readonly __wbg_get_wasmw3i_global_weather: (a: number) => number;
  readonly __wbg_set_wasmw3i_global_weather: (a: number, b: number) => void;
  readonly __wbg_get_wasmw3i_sound_environment: (a: number) => [number, number];
  readonly __wbg_set_wasmw3i_sound_environment: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmw3i_light_environment_tileset: (a: number) => number;
  readonly __wbg_set_wasmw3i_light_environment_tileset: (a: number, b: number) => void;
  readonly __wbg_get_wasmw3i_water_vertex_color: (a: number) => any;
  readonly __wbg_set_wasmw3i_water_vertex_color: (a: number, b: number) => void;
  readonly __wbg_get_wasmw3i_script_mode: (a: number) => number;
  readonly __wbg_set_wasmw3i_script_mode: (a: number, b: number) => void;
  readonly __wbg_get_wasmw3i_graphics_mode: (a: number) => number;
  readonly __wbg_set_wasmw3i_graphics_mode: (a: number, b: number) => void;
  readonly __wbg_get_wasmw3i_unknown1: (a: number) => number;
  readonly __wbg_set_wasmw3i_unknown1: (a: number, b: number) => void;
  readonly __wbg_get_wasmw3i_players: (a: number) => [number, number];
  readonly __wbg_set_wasmw3i_players: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmw3i_forces: (a: number) => [number, number];
  readonly __wbg_set_wasmw3i_forces: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmw3i_upgrade_availability_changes: (a: number) => [number, number];
  readonly __wbg_set_wasmw3i_upgrade_availability_changes: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmw3i_tech_availability_changes: (a: number) => [number, number];
  readonly __wbg_set_wasmw3i_tech_availability_changes: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmw3i_random_unit_tables: (a: number) => [number, number];
  readonly __wbg_set_wasmw3i_random_unit_tables: (a: number, b: number, c: number) => void;
  readonly __wbg_get_wasmw3i_random_item_tables: (a: number) => [number, number];
  readonly __wbg_set_wasmw3i_random_item_tables: (a: number, b: number, c: number) => void;
  readonly __wbg_set_wasmtechavailabilitychange_id: (a: number, b: any) => void;
  readonly __wbg_set_wasmupgradeavailabilitychange_id: (a: number, b: any) => void;
  readonly __wbg_get_wasmrandomitemtable_id: (a: number) => number;
  readonly __wbg_get_wasmrandomunit_chance: (a: number) => number;
  readonly __wbg_get_wasmrandomunittable_position: (a: number) => number;
  readonly __wbg_get_wasmtechavailabilitychange_player_flags: (a: number) => number;
  readonly __wbg_get_wasmupgradeavailabilitychange_player_flags: (a: number) => number;
  readonly __wbg_get_wasmupgradeavailabilitychange_availability: (a: number) => number;
  readonly __wbg_set_wasmw3i_build_version: (a: number, b: number) => void;
  readonly __wbg_get_wasmtechavailabilitychange_id: (a: number) => any;
  readonly __wbg_get_wasmupgradeavailabilitychange_id: (a: number) => any;
  readonly __wbg_get_wasmw3i_build_version: (a: number) => any;
  readonly __wbg_set_wasmrandomitemtable_name: (a: number, b: number, c: number) => void;
  readonly __wbg_set_wasmrandomunittable_name: (a: number, b: number, c: number) => void;
  readonly __wbg_wasmtechavailabilitychange_free: (a: number, b: number) => void;
  readonly __wbg_set_wasmrandomitemtable_id: (a: number, b: number) => void;
  readonly __wbg_set_wasmrandomunit_chance: (a: number, b: number) => void;
  readonly __wbg_set_wasmrandomunittable_position: (a: number, b: number) => void;
  readonly __wbg_set_wasmtechavailabilitychange_player_flags: (a: number, b: number) => void;
  readonly __wbg_set_wasmupgradeavailabilitychange_player_flags: (a: number, b: number) => void;
  readonly __wbg_set_wasmupgradeavailabilitychange_availability: (a: number, b: number) => void;
  readonly __wbg_get_wasmrandomitemtable_name: (a: number) => [number, number];
  readonly __wbg_get_wasmrandomunittable_name: (a: number) => [number, number];
  readonly __wbg_wasmmapinfo_free: (a: number, b: number) => void;
  readonly __wbg_get_wasmmapinfo_map_info: (a: number) => number;
  readonly __wbg_set_wasmmapinfo_map_info: (a: number, b: number) => void;
  readonly __wbg_get_wasmmapinfo_images: (a: number) => [number, number];
  readonly __wbg_set_wasmmapinfo_images: (a: number, b: number, c: number) => void;
  readonly wasmmapinfo_new: (a: any) => number;
  readonly __wbg_wasmimage_free: (a: number, b: number) => void;
  readonly __wbg_get_wasmimage_width: (a: number) => number;
  readonly __wbg_set_wasmimage_width: (a: number, b: number) => void;
  readonly __wbg_get_wasmimage_height: (a: number) => number;
  readonly __wbg_set_wasmimage_height: (a: number, b: number) => void;
  readonly __wbg_get_wasmimage_data: (a: number) => any;
  readonly __wbg_set_wasmimage_data: (a: number, b: any) => void;
  readonly __wbg_wasmwts_free: (a: number, b: number) => void;
  readonly __wbg_get_wasmwts_string_map: (a: number) => any;
  readonly __wbg_set_wasmwts_string_map: (a: number, b: any) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
