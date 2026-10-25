export type Language = 'en' | 'pt';

export interface StatusResponse {
  version: string;
  language: string;
  games_count: number;
  launch_behavior: string;
  dualsense_extension_enabled?: boolean;
  dualsense_autostart?: boolean;
  dualsense_connected: boolean;
  dualsense_udp_enabled: boolean;
  scan_directories?: string[];
  debug_console_visible?: boolean;
  steam_shortcuts_path?: string;
  steam_vdf_is_real?: boolean;
  steam_vdf_is_custom?: boolean;
}

export interface SteamPathConfig {
  path: string;
  is_real: boolean;
  is_custom: boolean;
  candidates: string[];
}

export interface Game {
  id: string;
  uuid?: string;
  title: string;
  platform?: string;
  platform_name?: string;
  path: string;
  custom_exe?: string;
  custom_args?: string;
  shortcut_id?: number;
  hidden?: boolean;
  art?: {
    cover?: string;
    hero?: string;
    logo?: string;
    wide?: string;
    icon?: string;
  };
}

export interface EmulatorInfo {
  key: string;
  name: string;
  download_url?: string;
  default_args: string;
  fullscreen_args: string;
  detected_path?: string;
  configured_path?: string;
}

export interface AchievementUnlockEvent {
  id: string;
  game_id: string;
  achievement_id: string;
  title: string;
  description: string;
  icon: string;
  source: string;
  unlocked_at: string;
  steam_notified: boolean;
}

// ==========================================
// DUALSENSE & ADAPTIVE TRIGGERS TYPES
// ==========================================

export type TriggerMode =
  | 'off'
  | 'feedback'
  | 'weapon'
  | 'bow'
  | 'galloping'
  | 'machine_gun'
  | 'vibration'
  | 'custom_zone';

export type TriggerEffect =
  | { mode: 'off' }
  | { mode: 'feedback'; start_position: number; force: number }
  | { mode: 'weapon'; start_position: number; end_position: number; force: number }
  | { mode: 'bow'; start_position: number; end_position: number; force: number; snap_force: number }
  | { mode: 'galloping'; start_position: number; end_position: number; first_foot: number; second_foot: number; frequency: number }
  | { mode: 'machine_gun'; start_position: number; end_position: number; strength: number; frequency: number }
  | { mode: 'vibration'; position: number; amplitude: number; frequency: number }
  | { mode: 'custom_zone'; zones: number[] };

export interface DualTriggerConfig {
  left: TriggerEffect;
  right: TriggerEffect;
}

export interface RgbColor {
  r: number;
  g: number;
  b: number;
}

export type PlayerLed = 'off' | 'player1' | 'player2' | 'player3' | 'player4' | 'all';
export type MicLed = 'off' | 'solid' | 'pulse';

export type LightbarMode =
  | { mode: 'off' }
  | { mode: 'static'; color: RgbColor }
  | { mode: 'breathing'; color: RgbColor; speed: number }
  | { mode: 'rainbow'; speed: number }
  | { mode: 'battery_level' };

export interface LightingConfig {
  lightbar: LightbarMode;
  player_led: PlayerLed;
  mic_led: MicLed;
}

export type EmulationMode = 'native' | 'xbox360' | 'dualshock4';

export interface GyroSettings {
  enabled: boolean;
  sensitivity: number;
  deadzone: number;
  invert_x: boolean;
  invert_y: boolean;
}

export interface EmulationConfig {
  mode: EmulationMode;
  gyro: GyroSettings;
}

export interface TriggerProfile {
  id: string;
  name: string;
  description: string;
  triggers: DualTriggerConfig;
  lightbar: RgbColor;
}

export interface BatteryState {
  percentage: number;
  is_charging: boolean;
  is_full: boolean;
}

export interface UdpServerStats {
  running: boolean;
  port: number;
  packets_received: number;
  last_packet_time: number | null;
  last_sender: string | null;
}

export interface DualSenseStatus {
  extension_enabled: boolean;
  autostart_on_launch: boolean;
  connected: boolean;
  name: string;
  connection_type: 'usb' | 'bluetooth';
  battery: BatteryState;
  triggers: DualTriggerConfig;
  lighting: LightingConfig;
  emulation: EmulationConfig;
  udp_stats: UdpServerStats;
  profiles: TriggerProfile[];
  mic_muted: boolean;
  audio_device_detected: boolean;
  speaker_volume: number;
}
