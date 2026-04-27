local wezterm = require 'wezterm'
local act = wezterm.action

local config = wezterm.config_builder()

local state_path = wezterm.home_dir .. '/.benterm-state.json'
local target = wezterm.target_triple or ''
local is_windows = target:find('windows', 1, true) ~= nil
local click_open_extensions = 'html?|pdf|md|txt|json|csv|ya?ml|toml|log|png|jpe?g|webp|gif|svg|zip|tar|gz|tgz|xz'
local paste_undo_window_seconds = 30
local paste_undo_max_chars = 200000
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

local function decode_percent_escapes(s)
  return (s:gsub('%%(%x%x)', function(hex)
    return string.char(tonumber(hex, 16))
  end))
end

local function resolve_clicked_path(uri_path, pane)
  local path = decode_percent_escapes(uri_path or '')
  path = path:gsub('^%s+', ''):gsub('%s+$', '')
  path = path:gsub('^[<%(%[{]+', ''):gsub('[>%)%]}:,;]+$', '')
  if path == '' then
    return nil
  end

  path = path:gsub('^(.-%.[^\\/:]+):%d+:%d+$', '%1')
  path = path:gsub('^(.-%.[^\\/:]+):%d+$', '%1')

  if is_windows and path:match('^/[A-Za-z]:[\\/]') then
    path = path:sub(2)
  end

  if path == '~' or path:match('^~[\\/]') then
    return wezterm.home_dir .. path:sub(2)
  end

  if path:match('^[A-Za-z]:[\\/]') or path:match('^\\\\') or path:match('^/') then
    return path
  end

  if path:match('^%.[\\/]') then
    path = path:sub(3)
  end

  local cwd = pane:get_current_working_dir()
  if cwd and cwd.scheme == 'file' and type(cwd.file_path) == 'string' and cwd.file_path ~= '' then
    local base = cwd.file_path
    if is_windows and base:match('^/[A-Za-z]:[\\/]') then
      base = base:sub(2)
    end
    local sep = base:find('\\', 1, true) and '\\' or '/'
    if base:sub(-1) ~= '\\' and base:sub(-1) ~= '/' then
      base = base .. sep
    end
    return base .. path
  end

  return path
end

wezterm.on('open-uri', function(window, pane, uri)
  local raw_path = uri:match '^benpath:(.+)$'
  if not raw_path then
    return
  end

  local path = resolve_clicked_path(raw_path, pane)
  if not path then
    return false
  end

  wezterm.open_with(path)
  return false
end)

local function get_clipboard_text()
  if is_windows then
    local ok, stdout, _ = wezterm.run_child_process {
      'powershell.exe',
      '-NoProfile',
      '-NonInteractive',
      '-Command',
      "try { $t = Get-Clipboard -Raw -ErrorAction Stop } catch { $t = $null }; if ($null -ne $t) { [Console]::Out.Write($t) }",
    }
    if not ok then
      return nil
    end
    return stdout or ''
  end

  local commands = {
    { 'sh', '-lc', "command -v wl-paste >/dev/null 2>&1 && wl-paste --no-newline 2>/dev/null || true" },
    { 'sh', '-lc', "command -v xclip >/dev/null 2>&1 && xclip -selection clipboard -o 2>/dev/null || true" },
    { 'sh', '-lc', "command -v xsel >/dev/null 2>&1 && xsel --clipboard --output 2>/dev/null || true" },
    { 'sh', '-lc', "command -v pbpaste >/dev/null 2>&1 && pbpaste || true" },
  }

  for _, cmd in ipairs(commands) do
    local ok, stdout, _ = wezterm.run_child_process(cmd)
    if ok and type(stdout) == 'string' and stdout ~= '' then
      return stdout
    end
  end

  return nil
end

local function now_epoch_seconds()
  return tonumber(wezterm.time.now():format '%s') or 0
end

local paste_state_by_pane_id = {}

local function state_for_pane(pane)
  local id = pane:pane_id()
  local st = paste_state_by_pane_id[id]
  if not st then
    st = { undo = {}, redo = {}, last_paste_s = 0 }
    paste_state_by_pane_id[id] = st
  end
  return st
end

local function char_len(s)
  if utf8 and utf8.len then
    local n = utf8.len(s)
    if n then
      return n
    end
  end
  return #s
end

local function send_back_delete(pane, count)
  local chunk = 4096
  local bs = string.char(0x08)
  while count > 0 do
    local n = math.min(count, chunk)
    pane:send_text(string.rep(bs, n))
    count = count - n
  end
end

local smart_paste = wezterm.action_callback(function(window, pane)
  local text = get_clipboard_text()
  window:perform_action(act.PasteFrom 'Clipboard', pane)

  if not text or text == '' then
    return
  end

  local len = char_len(text)
  if len > paste_undo_max_chars then
    return
  end

  local st = state_for_pane(pane)
  table.insert(st.undo, { text = text, len = len })
  st.redo = {}
  st.last_paste_s = now_epoch_seconds()
end)

local undo_paste = wezterm.action_callback(function(window, pane)
  local st = state_for_pane(pane)
  local age = now_epoch_seconds() - (st.last_paste_s or 0)
  local entry = st.undo[#st.undo]

  if age > paste_undo_window_seconds or not entry then
    window:perform_action(act.SendKey { key = 'z', mods = 'CTRL' }, pane)
    return
  end

  send_back_delete(pane, entry.len)
  table.remove(st.undo)
  table.insert(st.redo, entry)
end)

local redo_paste = wezterm.action_callback(function(window, pane)
  local st = state_for_pane(pane)
  local entry = st.redo[#st.redo]
  if not entry then
    return
  end

  pane:send_paste(entry.text)
  table.remove(st.redo)
  table.insert(st.undo, entry)
  st.last_paste_s = now_epoch_seconds()
end)

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

config.hyperlink_rules = (function()
  local rules = wezterm.default_hyperlink_rules()

  table.insert(rules, {
    regex = [=[["']([A-Za-z]:(?:[\/][^\/\s<>"'`|:*?]+)+[\/]?)(?::\d+(?::\d+)?)?["']]=],
    format = 'benpath:$1',
    highlight = 1,
  })

  table.insert(rules, {
    regex = [[\b([A-Za-z]:(?:[\/][^\/\s<>"'`|:*?]+)+[\/]?)(?::\d+(?::\d+)?)?\b]],
    format = 'benpath:$1',
    highlight = 1,
  })

  table.insert(rules, {
    regex = [=[["'](\\[^\/\s<>"'`|:*?]+(?:\[^\/\s<>"'`|:*?]+)+[\/]?)(?::\d+(?::\d+)?)?["']]=],
    format = 'benpath:$1',
    highlight = 1,
  })

  table.insert(rules, {
    regex = [[\b((?:\./|\.\./|~/)[^\s<>"'`|:*?]+[\/]?)(?::\d+(?::\d+)?)?\b]],
    format = 'benpath:$1',
    highlight = 1,
  })

  table.insert(rules, {
    regex = [[(?<![/\\])\b([0-9A-Za-z][0-9A-Za-z._-]*\.(?i:]] .. click_open_extensions .. [[))(?::\d+(?::\d+)?)?\b(?![/\\])]],
    format = 'benpath:$1',
    highlight = 1,
  })

  return rules
end)()

config.keys = {
  { key = 't', mods = 'CTRL|ALT', action = cycle_theme },
  { key = 'T', mods = 'CTRL|ALT|SHIFT', action = cycle_theme },
  { key = 'mapped:t', mods = 'CTRL|ALT', action = cycle_theme },
  { key = 'mapped:T', mods = 'CTRL|ALT|SHIFT', action = cycle_theme },
  { key = 'z', mods = 'CTRL', action = undo_paste },
  { key = 'Z', mods = 'CTRL|SHIFT', action = redo_paste },
  { key = 'mapped:z', mods = 'CTRL', action = undo_paste },
  { key = 'mapped:Z', mods = 'CTRL|SHIFT', action = redo_paste },
  { key = '-', mods = 'CTRL', action = act.DecreaseFontSize },
  { key = '=', mods = 'CTRL', action = act.IncreaseFontSize },
  { key = '0', mods = 'CTRL', action = act.ResetFontSize },
}

if is_windows then
  table.insert(config.keys, { key = 'v', mods = 'CTRL', action = smart_paste })
  table.insert(config.keys, { key = 'mapped:v', mods = 'CTRL', action = smart_paste })
  table.insert(config.keys, { key = 'V', mods = 'CTRL|SHIFT', action = act.PasteFrom 'Clipboard' })
  table.insert(config.keys, { key = 'v', mods = 'CTRL|SHIFT', action = act.PasteFrom 'Clipboard' })
  table.insert(config.keys, { key = 'Insert', mods = 'SHIFT', action = smart_paste })
else
  table.insert(config.keys, { key = 'V', mods = 'CTRL|SHIFT', action = smart_paste })
  table.insert(config.keys, { key = 'v', mods = 'CTRL|SHIFT', action = smart_paste })
  table.insert(config.keys, { key = 'Insert', mods = 'SHIFT', action = smart_paste })
  table.insert(config.keys, { key = 'v', mods = 'ALT', action = act.PasteFrom 'Clipboard' })
end

if wezterm.target_triple and wezterm.target_triple:find('windows', 1, true) then
  config.win32_system_backdrop = 'Disable'
end

return config
