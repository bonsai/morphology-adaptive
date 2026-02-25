/* tslint:disable */
/* eslint-disable */

export class GameState {
    free(): void;
    [Symbol.dispose](): void;
    get_current_time(): number;
    get_lap(): number;
    get_rotation_y(): number;
    get_speed(): number;
    get_x(): number;
    get_y(): number;
    get_z(): number;
    is_completed(): boolean;
    constructor(total_laps: number);
    start_race(now: number): void;
    update(delta: number, now: number, keys: string[]): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_gamestate_free: (a: number, b: number) => void;
    readonly gamestate_get_current_time: (a: number) => number;
    readonly gamestate_get_lap: (a: number) => number;
    readonly gamestate_get_rotation_y: (a: number) => number;
    readonly gamestate_get_speed: (a: number) => number;
    readonly gamestate_get_x: (a: number) => number;
    readonly gamestate_get_y: (a: number) => number;
    readonly gamestate_get_z: (a: number) => number;
    readonly gamestate_is_completed: (a: number) => number;
    readonly gamestate_new: (a: number) => number;
    readonly gamestate_start_race: (a: number, b: number) => void;
    readonly gamestate_update: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __externref_table_alloc: () => number;
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
