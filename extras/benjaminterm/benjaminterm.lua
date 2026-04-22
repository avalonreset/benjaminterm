local wezterm = require 'wezterm'
local act = wezterm.action

local config = wezterm.config_builder()

local state_path = wezterm.home_dir .. '/.benjaminterm-state.json'
local builtin_schemes = wezterm.color.get_builtin_schemes()

local function read_file(path)
  local f = io.open(path, 'rb')
  if not f then
    return nil
  end
  local s = f:read '*a'
  f:close()
  return s
end

local function write_file_atomic(path, data)
  local tmp = path .. '.tmp'
  local f = io.open(tmp, 'wb')
  if not f then
    return false
  end
  f:write(data)
  f:close()
  pcall(os.remove, path)
  local ok = os.rename(tmp, path)
  if not ok then
    pcall(os.remove, tmp)
    return false
  end
  return true
end

local function load_state()
  local s = read_file(state_path)
  if not s or s == '' then
    return {}
  end
  local ok, decoded = pcall(wezterm.json_parse, s)
  if ok and type(decoded) == 'table' then
    return decoded
  end
  return {}
end

local function save_state(st)
  local ok, json = pcall(wezterm.json_encode, st)
  if ok and type(json) == 'string' then
    pcall(write_file_atomic, state_path, json)
  end
end

local function is_black_background(value)
  if type(value) ~= 'string' then
    return false
  end
  local v = value:lower()
  return v == '#000' or v == '#000000' or v == '#000000ff' or v == 'rgb:0000/0000/0000'
end

local hacker_schemes = {}
for name, scheme in pairs(builtin_schemes) do
  if type(scheme) == 'table' and is_black_background(scheme.background) then
    table.insert(hacker_schemes, name)
  end
end

table.sort(hacker_schemes, function(a, b)
  return a:lower() < b:lower()
end)

if #hacker_schemes == 0 then
  hacker_schemes = { 'Builtin Dark' }
end

local scheme_set = {}
for _, name in ipairs(hacker_schemes) do
  scheme_set[name] = true
end

local state = load_state()
local rng_seeded = false

local function seed_rng_once(seed_hint)
  if rng_seeded then
    return
  end
  local now_s = tonumber(wezterm.time.now():format '%s') or 0
  local micros = tonumber(wezterm.time.now():format '%f') or 0
  math.randomseed((now_s * 1000003 + micros * 9176 + (seed_hint or 0) * 7919) % 2147483647)
  math.random()
  math.random()
  rng_seeded = true
end

local function shuffle_bag(previous)
  local bag = {}
  for _, name in ipairs(hacker_schemes) do
    table.insert(bag, name)
  end
  for i = #bag, 2, -1 do
    local j = math.random(i)
    bag[i], bag[j] = bag[j], bag[i]
  end
  if previous and #bag > 1 and bag[1] == previous then
    local j = math.random(2, #bag)
    bag[1], bag[j] = bag[j], bag[1]
  end
  return bag
end

local function bag_is_valid(bag)
  if type(bag) ~= 'table' or #bag ~= #hacker_schemes then
    return false
  end
  local seen = {}
  for _, name in ipairs(bag) do
    if type(name) ~= 'string' or not scheme_set[name] or seen[name] then
      return false
    end
    seen[name] = true
  end
  return true
end

local function pick_scheme(seed_hint)
  seed_rng_once(seed_hint)

  local bag = state.theme_bag
  local idx = tonumber(state.theme_bag_index) or 1
  if not bag_is_valid(bag) or idx > #bag then
    bag = shuffle_bag(state.last_random_scheme)
    idx = 1
  end

  local picked = bag[idx]
  idx = idx + 1

  if idx > #bag then
    bag = shuffle_bag(picked)
    idx = 1
  end

  state.last_random_scheme = picked
  state.theme_bag = bag
  state.theme_bag_index = idx
  save_state(state)
  return picked
end

local function make_font()
  return wezterm.font_with_fallback {
    '0xProto',
  }
end

local function apply_window_defaults(window)
  local overrides = window:get_config_overrides()
  if overrides then
    return
  end

  window:set_config_overrides {
    color_scheme = pick_scheme(window:window_id()),
    font = make_font(),
    font_size = 16.0,
    adjust_window_size_when_changing_font_size = false,
  }
end

wezterm.on('window-config-reloaded', function(window, pane)
  apply_window_defaults(window)
end)

local cycle_theme = wezterm.action_callback(function(window, pane)
  local overrides = window:get_config_overrides() or {}
  local current = overrides.color_scheme or config.color_scheme
  local idx = 1
  for i, name in ipairs(hacker_schemes) do
    if name == current then
      idx = i
      break
    end
  end

  local next_name = hacker_schemes[(idx % #hacker_schemes) + 1]
  overrides.color_scheme = next_name
  overrides.colors = nil
  window:set_config_overrides(overrides)

  state.last_random_scheme = next_name
  state.theme_bag = nil
  state.theme_bag_index = nil
  save_state(state)
end)

config.font = make_font()
config.font_dirs = {
  'fonts',
  '../../assets/fonts',
}
config.font_size = 16.0
config.adjust_window_size_when_changing_font_size = false
config.harfbuzz_features = { 'calt=0', 'clig=0', 'liga=0' }

config.color_scheme = pick_scheme(0)
config.window_background_opacity = 1.0
config.enable_tab_bar = true
config.hide_tab_bar_if_only_one_tab = true
config.use_fancy_tab_bar = true
config.default_cursor_style = 'BlinkingBlock'
config.notification_handling = 'SuppressFromFocusedPane'

config.keys = {
  { key = 't', mods = 'CTRL|ALT', action = cycle_theme },
  { key = 'T', mods = 'CTRL|ALT|SHIFT', action = cycle_theme },
  { key = 'mapped:t', mods = 'CTRL|ALT', action = cycle_theme },
  { key = 'mapped:T', mods = 'CTRL|ALT|SHIFT', action = cycle_theme },
  { key = '-', mods = 'CTRL', action = act.DecreaseFontSize },
  { key = '=', mods = 'CTRL', action = act.IncreaseFontSize },
  { key = '0', mods = 'CTRL', action = act.ResetFontSize },
}

if wezterm.target_triple and wezterm.target_triple:find('windows', 1, true) then
  config.win32_system_backdrop = 'Disable'
end

return config
