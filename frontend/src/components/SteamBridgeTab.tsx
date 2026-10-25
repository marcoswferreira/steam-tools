import { useState } from 'react';
import { useTranslation } from '../../i18n';
import { Game, EmulatorInfo } from '../../types/models';
import {
  RefreshCw,
  FolderSearch,
  FolderCog,
  Plus,
  Trash2,
  X,
  Compass,
  CheckCircle2,
  AlertTriangle,
  RotateCcw,
  Gamepad2,
  Edit3,
  BellRing,
  Image as ImageIcon,
  ExternalLink,
} from 'lucide-react';

interface SteamBridgeTabProps {
  games: Game[];
  scanDirectories: string[];
  steamShortcutsPath?: string;
  steamVdfIsReal?: boolean;
  steamVdfIsCustom?: boolean;
  onSyncSteam: () => void;
  onScanGames: (paths?: string[]) => void;
  onUpdateScanDirectories: (paths: string[]) => void;
  onRemoveGame?: (id: string, title: string) => void;
  onConfigureSteamPath?: (path: string | null) => Promise<boolean>;
  onRefreshGames?: () => void;
  onShowToast?: (msg: string, type?: 'info' | 'success' | 'error') => void;
}

export function SteamBridgeTab({
  games,
  scanDirectories,
  steamShortcutsPath,
  steamVdfIsReal,
  steamVdfIsCustom,
  onSyncSteam,
  onScanGames,
  onUpdateScanDirectories,
  onRemoveGame,
  onConfigureSteamPath,
  onRefreshGames,
  onShowToast,
}: SteamBridgeTabProps) {
  const { t, lang } = useTranslation();
  const [cdpEnabled, setCdpEnabled] = useState(true);
  const [tagsEnabled, setTagsEnabled] = useState(true);
  const [modalOpen, setModalOpen] = useState(false);
  const [newPath, setNewPath] = useState('');

  // Steam Location modal state
  const [steamModalOpen, setSteamModalOpen] = useState(false);
  const [steamInputPath, setSteamInputPath] = useState('');
  const [candidates, setCandidates] = useState<string[]>([]);
  const [loadingCandidates, setLoadingCandidates] = useState(false);

  // Emulators modal state
  const [emulatorsModalOpen, setEmulatorsModalOpen] = useState(false);
  const [emulators, setEmulators] = useState<EmulatorInfo[]>([]);
  const [loadingEmulators, setLoadingEmulators] = useState(false);
  const [emuCustomPaths, setEmuCustomPaths] = useState<Record<string, string>>({});

  // Edit Game modal state
  const [editGameModalOpen, setEditGameModalOpen] = useState(false);
  const [editingGame, setEditingGame] = useState<Game | null>(null);

  // Testing CDP Toast state
  const [testingCdp, setTestingCdp] = useState(false);

  const truncate = (str: string, len: number) => {
    if (!str || str.length <= len) return str;
    const parts = str.split(/[/\\]/);
    const filename = parts.pop();
    return `.../${filename}`;
  };

  const handleAddDirectory = (e: React.FormEvent) => {
    e.preventDefault();
    const trimmed = newPath.trim();
    if (!trimmed || scanDirectories.includes(trimmed)) return;
    const updated = [...scanDirectories, trimmed];
    onUpdateScanDirectories(updated);
    setNewPath('');
  };

  const handleRemoveDirectory = (index: number) => {
    const updated = scanDirectories.filter((_, i) => i !== index);
    onUpdateScanDirectories(updated);
  };

  const handleScanFromModal = () => {
    onScanGames(scanDirectories);
    setModalOpen(false);
  };

  const handleOpenSteamModal = async () => {
    setSteamInputPath(steamShortcutsPath || '');
    setSteamModalOpen(true);
    setLoadingCandidates(true);
    try {
      const res = await fetch('/api/steam/path');
      if (res.ok) {
        const data = await res.json();
        setCandidates(data.candidates || []);
        if (data.path) {
          setSteamInputPath(data.path);
        }
      }
    } catch {
      // ignore
    } finally {
      setLoadingCandidates(false);
    }
  };

  const handleSaveSteamPath = async (e: React.FormEvent) => {
    e.preventDefault();
    const ok = await onConfigureSteamPath?.(steamInputPath.trim() || null);
    if (ok) {
      setSteamModalOpen(false);
    }
  };

  const handleResetSteamPath = async () => {
    const ok = await onConfigureSteamPath?.(null);
    if (ok) {
      setSteamModalOpen(false);
    }
  };

  const handleOpenEmulatorsModal = async () => {
    setEmulatorsModalOpen(true);
    setLoadingEmulators(true);
    try {
      const res = await fetch('/api/emulators');
      if (res.ok) {
        const data: EmulatorInfo[] = await res.json();
        setEmulators(data);
        const map: Record<string, string> = {};
        for (const emu of data) {
          map[emu.key] = emu.configured_path || '';
        }
        setEmuCustomPaths(map);
      }
    } catch (_) {
    } finally {
      setLoadingEmulators(false);
    }
  };

  const handleSaveEmulator = async (key: string) => {
    const customPath = emuCustomPaths[key] || '';
    try {
      const res = await fetch('/api/emulators/configure', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ key, path: customPath }),
      });
      if (res.ok) {
        onShowToast?.(t.toast_emulator_saved, 'success');
        await handleOpenEmulatorsModal();
      }
    } catch (_) {
      onShowToast?.(t.toast_error, 'error');
    }
  };

  const handleOpenEditGame = (game: Game) => {
    setEditingGame({
      ...game,
      art: {
        cover: game.art?.cover || '',
        hero: game.art?.hero || '',
        logo: game.art?.logo || '',
        icon: game.art?.icon || '',
      },
    });
    setEditGameModalOpen(true);
  };

  const handleSaveGameEdit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!editingGame) return;
    try {
      const res = await fetch('/api/games/update', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(editingGame),
      });
      if (res.ok) {
        onShowToast?.(t.toast_game_updated, 'success');
        setEditGameModalOpen(false);
        onRefreshGames?.();
      } else {
        onShowToast?.(t.toast_error, 'error');
      }
    } catch (_) {
      onShowToast?.(t.toast_error, 'error');
    }
  };

  const handleTestCdpToast = async () => {
    setTestingCdp(true);
    try {
      const res = await fetch('/api/achievements/test-toast', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          title: 'Steam Companion Master',
          description:
            lang === 'pt'
              ? 'Conquista sincronizada na Steam com sucesso!'
              : 'Achievement synced with Steam successfully!',
          icon_url:
            'https://community.cloudflare.steamstatic.com/public/images/apps/480/achievement_1.jpg',
          app_id: 480,
        }),
      });
      if (res.ok) {
        const data = await res.json();
        onShowToast?.(
          data.steam_cdp_connected
            ? t.toast_cdp_sent
            : `${t.toast_cdp_sent} (Log local)`,
          'success'
        );
      } else {
        onShowToast?.(t.toast_error, 'error');
      }
    } catch (_) {
      onShowToast?.(t.toast_error, 'error');
    } finally {
      setTestingCdp(false);
    }
  };

  return (
    <section className="tab-panel">
      <div className="panel-header">
        <h1 className="panel-title">{t.steam_bridge_title}</h1>
        <p className="panel-desc">{t.steam_bridge_desc}</p>
      </div>

      {/* Target Steam Shortcuts Destination Card */}
      <div
        className="deck-card"
        style={{
          marginBottom: '20px',
          padding: '14px 18px',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
          gap: '16px',
          flexWrap: 'wrap',
          borderLeft: steamVdfIsCustom
            ? '4px solid #3b82f6'
            : steamVdfIsReal
              ? '4px solid var(--deck-green)'
              : '4px solid #f59e0b',
        }}
      >
        <div style={{ display: 'flex', flexDirection: 'column', gap: '4px', minWidth: '260px', flex: 1 }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            <span style={{ fontSize: '11px', fontWeight: 700, color: '#e5e7eb', textTransform: 'uppercase', letterSpacing: '0.5px' }}>
              {t.current_steam_vdf}
            </span>
            <span
              className="status-badge active"
              style={{
                padding: '2px 8px',
                borderRadius: '3px',
                fontSize: '10px',
                fontWeight: 700,
                backgroundColor: steamVdfIsCustom
                  ? 'rgba(59, 130, 246, 0.15)'
                  : steamVdfIsReal
                    ? 'rgba(92, 191, 71, 0.15)'
                    : 'rgba(245, 158, 11, 0.15)',
                border: `1px solid ${
                  steamVdfIsCustom
                    ? 'rgba(59, 130, 246, 0.4)'
                    : steamVdfIsReal
                      ? 'rgba(92, 191, 71, 0.4)'
                      : 'rgba(245, 158, 11, 0.4)'
                }`,
                color: steamVdfIsCustom
                  ? '#60a5fa'
                  : steamVdfIsReal
                    ? 'var(--deck-green)'
                    : '#f59e0b',
              }}
            >
              {steamVdfIsCustom
                ? t.badge_steam_custom
                : steamVdfIsReal
                  ? t.badge_steam_detected
                  : t.badge_steam_not_found}
            </span>
          </div>
          <div
            className="mono-cell"
            style={{
              fontSize: '12px',
              color: steamVdfIsReal || steamVdfIsCustom ? 'var(--deck-text-secondary)' : '#f87171',
              wordBreak: 'break-all',
            }}
            title={steamShortcutsPath}
          >
            {steamShortcutsPath || (lang === 'pt' ? 'Nenhum caminho detectado' : 'No path detected')}
          </div>
        </div>

        <button
          className="deck-btn focusable"
          onClick={handleOpenSteamModal}
          tabIndex={19}
          style={{ display: 'inline-flex', alignItems: 'center', gap: '6px', padding: '8px 14px' }}
        >
          <Compass size={14} />
          <span>{t.btn_configure_steam_path}</span>
        </button>
      </div>

      <div className="bridge-action-bar" style={{ display: 'flex', gap: '12px', marginBottom: '20px', flexWrap: 'wrap' }}>
        <button className="deck-btn deck-btn-primary focusable" onClick={onSyncSteam} tabIndex={20}>
          <RefreshCw size={15} />
          <span>{t.btn_sync_shortcuts_now}</span>
        </button>
        <button className="deck-btn focusable" onClick={() => onScanGames()} tabIndex={21} title="Scan all saved directories">
          <FolderSearch size={15} />
          <span>{t.btn_scan_folders}</span>
        </button>
        <button className="deck-btn focusable" onClick={() => setModalOpen(true)} tabIndex={22} title="Choose custom folders">
          <FolderCog size={15} />
          <span>{t.btn_manage_folders}</span>
        </button>
        <button
          className="deck-btn focusable"
          onClick={handleOpenEmulatorsModal}
          tabIndex={23}
          title="Configure emulator executables for launching ROMs"
          style={{ display: 'inline-flex', alignItems: 'center', gap: '6px' }}
        >
          <Gamepad2 size={15} />
          <span>{t.btn_configure_emulators}</span>
        </button>
      </div>

      <div className="settings-group">
        <div className="setting-row">
          <div className="setting-info">
            <div className="setting-label">{t.setting_deterministic_appids}</div>
            <div className="setting-sub">{t.setting_deterministic_appids_sub}</div>
          </div>
          <div className="setting-control">
            <span
              className="status-badge active"
              style={{
                display: 'inline-block',
                padding: '3px 8px',
                borderRadius: '3px',
                fontSize: '10px',
                fontWeight: 700,
                backgroundColor: 'rgba(92, 191, 71, 0.15)',
                border: '1px solid rgba(92, 191, 71, 0.4)',
                color: 'var(--deck-green)',
              }}
            >
              {t.badge_enforced}
            </span>
          </div>
        </div>

        <div className="setting-row">
          <div className="setting-info">
            <div className="setting-label">{t.setting_cdp_listener}</div>
            <div className="setting-sub">{t.setting_cdp_listener_sub}</div>
          </div>
          <div className="setting-control" style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
            <button
              type="button"
              className="deck-btn focusable"
              onClick={handleTestCdpToast}
              disabled={testingCdp}
              style={{ padding: '4px 10px', fontSize: '11px', display: 'inline-flex', alignItems: 'center', gap: '5px' }}
              title="Dispatches a test achievement toast to Steam client via CDP"
            >
              <BellRing size={12} />
              <span>{testingCdp ? t.loading : t.btn_test_cdp_toast}</span>
            </button>
            <label className="deck-switch">
              <input
                type="checkbox"
                checked={cdpEnabled}
                onChange={(e) => setCdpEnabled(e.target.checked)}
                className="focusable"
                tabIndex={24}
              />
              <span className="switch-slider" />
            </label>
          </div>
        </div>

        <div className="setting-row">
          <div className="setting-info">
            <div className="setting-label">{t.setting_steam_tags}</div>
            <div className="setting-sub">{t.setting_steam_tags_sub}</div>
          </div>
          <div className="setting-control">
            <label className="deck-switch">
              <input
                type="checkbox"
                checked={tagsEnabled}
                onChange={(e) => setTagsEnabled(e.target.checked)}
                className="focusable"
                tabIndex={25}
              />
              <span className="switch-slider" />
            </label>
          </div>
        </div>
      </div>

      <div className="section-divider">
        <span className="divider-title">{t.discovered_games_title}</span>
        <span className="divider-meta">{games.length} items ready</span>
      </div>

      <div className="table-container">
        <table className="deck-table">
          <thead>
            <tr>
              <th style={{ width: '45px' }}>{t.th_cover}</th>
              <th>{t.th_title}</th>
              <th>{t.th_platform}</th>
              <th>{t.th_emulator}</th>
              <th>{t.th_appid}</th>
              <th>{t.th_status}</th>
              <th style={{ width: '150px', textAlign: 'center' }}>{t.th_actions}</th>
            </tr>
          </thead>
          <tbody>
            {games.length === 0 ? (
              <tr>
                <td
                  colSpan={7}
                  className="table-empty"
                  style={{ textAlign: 'center', padding: '36px 16px', color: 'var(--deck-text-muted)' }}
                >
                  {t.table_empty}
                </td>
              </tr>
            ) : (
              games.map((game, idx) => {
                const gameKey = game.id || (game as { uuid?: string }).uuid || game.path || String(idx);
                const coverImg = game.art?.cover || game.art?.icon;
                return (
                  <tr key={gameKey}>
                    <td style={{ textAlign: 'center', padding: '6px' }}>
                      {coverImg ? (
                        <img
                          src={coverImg}
                          alt={game.title}
                          style={{
                            width: '32px',
                            height: '44px',
                            objectFit: 'cover',
                            borderRadius: '3px',
                            border: '1px solid rgba(255, 255, 255, 0.1)',
                            backgroundColor: '#0f172a',
                          }}
                          onError={(e) => {
                            (e.target as HTMLElement).style.display = 'none';
                          }}
                        />
                      ) : (
                        <div
                          style={{
                            width: '32px',
                            height: '44px',
                            borderRadius: '3px',
                            backgroundColor: 'rgba(255, 255, 255, 0.05)',
                            display: 'flex',
                            alignItems: 'center',
                            justifyContent: 'center',
                            color: 'var(--deck-text-muted)',
                            margin: '0 auto',
                          }}
                        >
                          <Gamepad2 size={16} />
                        </div>
                      )}
                    </td>
                    <td style={{ color: '#ffffff', fontWeight: 600 }}>{game.title}</td>
                    <td>
                      <span className="platform-pill">
                        {game.platform_name || game.platform || 'PC'}
                      </span>
                    </td>
                    <td className="mono-cell" title={game.custom_exe || game.path}>
                      {truncate(game.custom_exe || (game.path.toLowerCase().endsWith('.exe') ? 'Native .exe' : game.path), 30)}
                    </td>
                    <td className="mono-cell">{game.shortcut_id || 'AUTO-CRC'}</td>
                    <td>
                      <span
                        className="status-badge active"
                        style={{
                          display: 'inline-flex',
                          alignItems: 'center',
                          gap: '6px',
                          padding: '4px 10px',
                          borderRadius: '4px',
                          fontSize: '11px',
                          fontWeight: 600,
                          backgroundColor: 'rgba(92, 191, 71, 0.15)',
                          border: '1px solid rgba(92, 191, 71, 0.35)',
                          color: 'var(--deck-green)',
                        }}
                      >
                        <span
                          style={{
                            width: '6px',
                            height: '6px',
                            borderRadius: '50%',
                            backgroundColor: 'var(--deck-green)',
                          }}
                        />
                        <span>{t.status_steam_ready}</span>
                      </span>
                    </td>
                    <td style={{ textAlign: 'center' }}>
                      <div style={{ display: 'inline-flex', gap: '6px' }}>
                        <button
                          className="deck-btn focusable"
                          style={{
                            padding: '5px 8px',
                            fontSize: '11px',
                            display: 'inline-flex',
                            alignItems: 'center',
                            gap: '4px',
                          }}
                          onClick={() => handleOpenEditGame(game)}
                          title={t.btn_edit_game}
                        >
                          <Edit3 size={13} />
                          <span>{t.btn_edit_game}</span>
                        </button>
                        <button
                          className="deck-btn deck-btn-danger focusable"
                          style={{
                            padding: '5px 8px',
                            fontSize: '11px',
                            display: 'inline-flex',
                            alignItems: 'center',
                            gap: '4px',
                            borderColor: 'rgba(239, 68, 68, 0.4)',
                            color: '#f87171',
                          }}
                          onClick={() => onRemoveGame?.(gameKey, game.title)}
                          title={t.btn_remove_game}
                        >
                          <Trash2 size={13} />
                        </button>
                      </div>
                    </td>
                  </tr>
                );
              })
            )}
          </tbody>
        </table>
      </div>

      {/* Point Steam Location Modal */}
      {steamModalOpen && (
        <div className="modal-overlay">
          <div className="modal-dialog" style={{ width: '640px' }}>
            <div className="modal-header" style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                <Compass size={18} style={{ color: '#3b82f6' }} />
                <span>{t.modal_steam_path_title}</span>
              </div>
              <button
                className="deck-btn focusable"
                style={{ padding: '4px', background: 'transparent', border: 'none' }}
                onClick={() => setSteamModalOpen(false)}
              >
                <X size={16} />
              </button>
            </div>

            <p style={{ fontSize: '13px', color: 'var(--deck-text-secondary)', lineHeight: 1.5, marginBottom: '16px' }}>
              {t.modal_steam_path_desc}
            </p>

            {/* List of Detected Steam candidate paths if available */}
            {candidates.length > 0 && (
              <div style={{ marginBottom: '16px' }}>
                <div style={{ fontSize: '12px', fontWeight: 600, color: '#e5e7eb', marginBottom: '8px' }}>
                  {t.steam_detected_candidates}
                </div>
                <div style={{ display: 'flex', flexDirection: 'column', gap: '6px', maxHeight: '140px', overflowY: 'auto' }}>
                  {candidates.map((cand, i) => (
                    <button
                      key={i}
                      type="button"
                      className="deck-btn focusable"
                      style={{
                        textAlign: 'left',
                        justifyContent: 'flex-start',
                        padding: '8px 12px',
                        fontSize: '11.5px',
                        borderColor: steamInputPath === cand ? 'var(--deck-blue)' : 'var(--deck-border)',
                        backgroundColor: steamInputPath === cand ? 'rgba(59, 130, 246, 0.15)' : 'var(--deck-card-bg)',
                      }}
                      onClick={() => setSteamInputPath(cand)}
                    >
                      <CheckCircle2 size={13} style={{ color: steamInputPath === cand ? '#60a5fa' : 'var(--deck-text-muted)', flexShrink: 0 }} />
                      <span className="mono-cell" style={{ wordBreak: 'break-all' }}>{cand}</span>
                    </button>
                  ))}
                </div>
              </div>
            )}

            {/* Steam Path Form */}
            <form onSubmit={handleSaveSteamPath}>
              <div style={{ marginBottom: '16px' }}>
                <label style={{ display: 'block', fontSize: '12px', fontWeight: 600, color: '#e5e7eb', marginBottom: '6px' }}>
                  {t.steam_path_label}
                </label>
                <input
                  type="text"
                  className="deck-input focusable"
                  style={{ width: '100%', fontFamily: 'var(--font-mono)', fontSize: '12.5px' }}
                  value={steamInputPath}
                  onChange={(e) => setSteamInputPath(e.target.value)}
                  placeholder={t.steam_path_placeholder}
                />
              </div>

              <div className="modal-footer" style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginTop: '20px' }}>
                <button
                  type="button"
                  className="deck-btn focusable"
                  style={{ display: 'inline-flex', alignItems: 'center', gap: '6px' }}
                  onClick={handleResetSteamPath}
                  title={t.btn_reset_steam_path}
                >
                  <RotateCcw size={13} />
                  <span>{t.btn_reset_steam_path}</span>
                </button>

                <div style={{ display: 'flex', gap: '8px' }}>
                  <button type="button" className="deck-btn focusable" onClick={() => setSteamModalOpen(false)}>
                    {t.btn_cancel}
                  </button>
                  <button type="submit" className="deck-btn deck-btn-primary focusable">
                    {t.btn_save_steam_path}
                  </button>
                </div>
              </div>
            </form>
          </div>
        </div>
      )}

      {/* Manage Scan Folders Modal */}
      {modalOpen && (
        <div className="modal-overlay">
          <div className="modal-dialog" style={{ width: '600px' }}>
            <div className="modal-header" style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              <span>{t.modal_folders_title}</span>
              <button
                className="deck-btn focusable"
                style={{ padding: '4px', background: 'transparent', border: 'none' }}
                onClick={() => setModalOpen(false)}
              >
                <X size={16} />
              </button>
            </div>

            <p style={{ fontSize: '12px', color: 'var(--deck-text-secondary)', marginBottom: '16px' }}>
              {t.modal_folders_desc}
            </p>

            {/* List of active folders */}
            <div style={{ display: 'flex', flexDirection: 'column', gap: '8px', marginBottom: '18px', maxHeight: '180px', overflowY: 'auto' }}>
              {scanDirectories.length === 0 ? (
                <div style={{ fontSize: '12px', color: 'var(--deck-text-muted)', padding: '12px', backgroundColor: 'var(--deck-card-bg)', borderRadius: '4px' }}>
                  {t.no_folders_configured}
                </div>
              ) : (
                scanDirectories.map((dir, idx) => (
                  <div
                    key={idx}
                    style={{
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'space-between',
                      backgroundColor: 'var(--deck-card-bg)',
                      border: '1px solid var(--deck-border)',
                      padding: '8px 12px',
                      borderRadius: '4px',
                    }}
                  >
                    <span className="mono-cell" style={{ color: 'var(--deck-text-primary)' }}>{dir}</span>
                    <button
                      className="deck-btn deck-btn-danger focusable"
                      style={{ padding: '4px 8px', fontSize: '11px' }}
                      onClick={() => handleRemoveDirectory(idx)}
                    >
                      <Trash2 size={12} />
                    </button>
                  </div>
                ))
              )}
            </div>

            {/* Add new folder form */}
            <form onSubmit={handleAddDirectory} style={{ display: 'flex', gap: '8px', marginBottom: '20px' }}>
              <input
                type="text"
                className="deck-input focusable"
                style={{ flex: 1 }}
                value={newPath}
                onChange={(e) => setNewPath(e.target.value)}
                placeholder={t.field_add_folder}
              />
              <button type="submit" className="deck-btn focusable">
                <Plus size={14} />
                <span>{t.btn_add_folder}</span>
              </button>
            </form>

            <div className="modal-footer" style={{ display: 'flex', justifyContent: 'space-between' }}>
              <button type="button" className="deck-btn focusable" onClick={() => setModalOpen(false)}>
                {t.btn_cancel}
              </button>
              <button
                type="button"
                className="deck-btn deck-btn-primary focusable"
                onClick={handleScanFromModal}
              >
                <FolderSearch size={14} />
                <span>{t.btn_scan_all_folders}</span>
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Emulators Configuration Modal */}
      {emulatorsModalOpen && (
        <div className="modal-overlay">
          <div
            className="modal-dialog"
            style={{
              width: '720px',
              maxHeight: '85vh',
              display: 'flex',
              flexDirection: 'column',
            }}
          >
            <div
              className="modal-header"
              style={{
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
              }}
            >
              <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                <Gamepad2 size={18} style={{ color: 'var(--deck-blue)' }} />
                <span>{t.modal_emulators_title}</span>
              </div>
              <button
                className="deck-btn focusable"
                style={{ padding: '4px', background: 'transparent', border: 'none' }}
                onClick={() => setEmulatorsModalOpen(false)}
              >
                <X size={16} />
              </button>
            </div>

            <p
              style={{
                fontSize: '13px',
                color: 'var(--deck-text-secondary)',
                marginBottom: '16px',
                lineHeight: 1.4,
              }}
            >
              {t.modal_emulators_desc}
            </p>

            <div
              style={{
                flex: 1,
                overflowY: 'auto',
                display: 'flex',
                flexDirection: 'column',
                gap: '12px',
                paddingRight: '4px',
              }}
            >
              {loadingEmulators ? (
                <div
                  style={{
                    padding: '24px',
                    textAlign: 'center',
                    color: 'var(--deck-text-muted)',
                  }}
                >
                  {t.loading}...
                </div>
              ) : (
                emulators.map((emu) => {
                  const isDetected = Boolean(
                    emu.detected_path || emu.configured_path
                  );
                  const effectivePath =
                    emu.configured_path || emu.detected_path || '';
                  return (
                    <div
                      key={emu.key}
                      style={{
                        backgroundColor: 'var(--deck-card-bg)',
                        border: '1px solid var(--deck-border)',
                        borderRadius: '6px',
                        padding: '12px 14px',
                      }}
                    >
                      <div
                        style={{
                          display: 'flex',
                          justifyContent: 'space-between',
                          alignItems: 'center',
                          marginBottom: '8px',
                        }}
                      >
                        <div
                          style={{
                            display: 'flex',
                            alignItems: 'center',
                            gap: '8px',
                          }}
                        >
                          <span
                            style={{
                              fontWeight: 600,
                              color: '#ffffff',
                              fontSize: '14px',
                            }}
                          >
                            {emu.name}
                          </span>
                          <span
                            className="status-badge active"
                            style={{
                              fontSize: '10px',
                              padding: '2px 6px',
                              borderRadius: '3px',
                              backgroundColor: isDetected
                                ? 'rgba(92, 191, 71, 0.15)'
                                : 'rgba(239, 68, 68, 0.15)',
                              color: isDetected ? 'var(--deck-green)' : '#f87171',
                              border: isDetected
                                ? '1px solid rgba(92, 191, 71, 0.4)'
                                : '1px solid rgba(239, 68, 68, 0.4)',
                            }}
                          >
                            {isDetected
                              ? t.emulator_detected
                              : t.emulator_not_detected}
                          </span>
                        </div>
                        {emu.download_url && (
                          <a
                            href={emu.download_url}
                            target="_blank"
                            rel="noreferrer"
                            style={{
                              fontSize: '11px',
                              color: '#60a5fa',
                              display: 'inline-flex',
                              alignItems: 'center',
                              gap: '4px',
                              textDecoration: 'none',
                            }}
                          >
                            <span>Site Oficial</span>
                            <ExternalLink size={11} />
                          </a>
                        )}
                      </div>

                      <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
                        <input
                          type="text"
                          className="deck-input focusable mono-cell"
                          style={{ flex: 1, fontSize: '12px' }}
                          placeholder={
                            effectivePath ||
                            (lang === 'pt'
                              ? 'Caminho do executável (ex: C:\\...\\pcsx2-qt.exe)'
                              : 'Path to executable')
                          }
                          value={
                            emuCustomPaths[emu.key] ?? emu.configured_path ?? ''
                          }
                          onChange={(e) => {
                            const val = e.target.value;
                            setEmuCustomPaths((prev) => ({
                              ...prev,
                              [emu.key]: val,
                            }));
                          }}
                        />
                        <button
                          type="button"
                          className="deck-btn focusable"
                          style={{ padding: '6px 12px', fontSize: '11.5px' }}
                          onClick={() => handleSaveEmulator(emu.key)}
                        >
                          <span>Salvar</span>
                        </button>
                      </div>
                    </div>
                  );
                })
              )}
            </div>

            <div
              className="modal-footer"
              style={{
                marginTop: '16px',
                display: 'flex',
                justifyContent: 'flex-end',
              }}
            >
              <button
                type="button"
                className="deck-btn deck-btn-primary focusable"
                onClick={() => setEmulatorsModalOpen(false)}
              >
                <span>Fechar</span>
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Edit Game Modal */}
      {editGameModalOpen && editingGame && (
        <div className="modal-overlay">
          <div
            className="modal-dialog"
            style={{
              width: '640px',
              maxHeight: '88vh',
              display: 'flex',
              flexDirection: 'column',
            }}
          >
            <div
              className="modal-header"
              style={{
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
              }}
            >
              <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                <Edit3 size={18} style={{ color: 'var(--deck-blue)' }} />
                <span>{t.modal_edit_game_title}</span>
              </div>
              <button
                className="deck-btn focusable"
                style={{ padding: '4px', background: 'transparent', border: 'none' }}
                onClick={() => setEditGameModalOpen(false)}
              >
                <X size={16} />
              </button>
            </div>

            <p
              style={{
                fontSize: '13px',
                color: 'var(--deck-text-secondary)',
                marginBottom: '16px',
                lineHeight: 1.4,
              }}
            >
              {t.modal_edit_game_desc}
            </p>

            <form
              onSubmit={handleSaveGameEdit}
              style={{
                flex: 1,
                overflowY: 'auto',
                display: 'flex',
                flexDirection: 'column',
                gap: '14px',
                paddingRight: '4px',
              }}
            >
              <div>
                <label
                  style={{
                    display: 'block',
                    fontSize: '12px',
                    fontWeight: 600,
                    color: '#e5e7eb',
                    marginBottom: '6px',
                  }}
                >
                  {t.field_game_title}
                </label>
                <input
                  type="text"
                  className="deck-input focusable"
                  style={{ width: '100%', fontSize: '13px' }}
                  value={editingGame.title}
                  onChange={(e) =>
                    setEditingGame({ ...editingGame, title: e.target.value })
                  }
                />
              </div>

              <div>
                <label
                  style={{
                    display: 'block',
                    fontSize: '12px',
                    fontWeight: 600,
                    color: '#e5e7eb',
                    marginBottom: '6px',
                  }}
                >
                  {t.field_custom_exe}
                </label>
                <input
                  type="text"
                  className="deck-input focusable mono-cell"
                  style={{ width: '100%', fontSize: '12px' }}
                  placeholder={
                    lang === 'pt'
                      ? 'Deixe em branco para usar o emulador padrão da plataforma'
                      : 'Leave empty to use platform default emulator'
                  }
                  value={editingGame.custom_exe || ''}
                  onChange={(e) =>
                    setEditingGame({
                      ...editingGame,
                      custom_exe: e.target.value,
                    })
                  }
                />
              </div>

              <div>
                <label
                  style={{
                    display: 'block',
                    fontSize: '12px',
                    fontWeight: 600,
                    color: '#e5e7eb',
                    marginBottom: '6px',
                  }}
                >
                  {t.field_custom_args}
                </label>
                <input
                  type="text"
                  className="deck-input focusable mono-cell"
                  style={{ width: '100%', fontSize: '12px' }}
                  placeholder={
                    lang === 'pt'
                      ? 'ex: -batch -fullscreen "{game}"'
                      : 'e.g. -batch -fullscreen "{game}"'
                  }
                  value={editingGame.custom_args || ''}
                  onChange={(e) =>
                    setEditingGame({
                      ...editingGame,
                      custom_args: e.target.value,
                    })
                  }
                />
              </div>

              <div
                style={{
                  borderTop: '1px solid var(--deck-border)',
                  paddingTop: '12px',
                  marginTop: '6px',
                }}
              >
                <div
                  style={{
                    display: 'flex',
                    alignItems: 'center',
                    gap: '6px',
                    fontSize: '12px',
                    fontWeight: 700,
                    color: '#60a5fa',
                    marginBottom: '10px',
                  }}
                >
                  <ImageIcon size={14} />
                  <span>Artes da Steam Grid (userdata/.../config/grid/)</span>
                </div>

                <div
                  style={{
                    display: 'grid',
                    gridTemplateColumns: '1fr 1fr',
                    gap: '10px',
                  }}
                >
                  <div>
                    <label
                      style={{
                        display: 'block',
                        fontSize: '11px',
                        color: 'var(--deck-text-secondary)',
                        marginBottom: '4px',
                      }}
                    >
                      {t.field_cover_art}
                    </label>
                    <input
                      type="text"
                      className="deck-input focusable mono-cell"
                      style={{ width: '100%', fontSize: '11.5px' }}
                      placeholder="Caminho do poster .png/.jpg"
                      value={editingGame.art?.cover || ''}
                      onChange={(e) =>
                        setEditingGame({
                          ...editingGame,
                          art: { ...editingGame.art, cover: e.target.value },
                        })
                      }
                    />
                  </div>

                  <div>
                    <label
                      style={{
                        display: 'block',
                        fontSize: '11px',
                        color: 'var(--deck-text-secondary)',
                        marginBottom: '4px',
                      }}
                    >
                      {t.field_hero_art}
                    </label>
                    <input
                      type="text"
                      className="deck-input focusable mono-cell"
                      style={{ width: '100%', fontSize: '11.5px' }}
                      placeholder="Caminho do banner hero .png/.jpg"
                      value={editingGame.art?.hero || ''}
                      onChange={(e) =>
                        setEditingGame({
                          ...editingGame,
                          art: { ...editingGame.art, hero: e.target.value },
                        })
                      }
                    />
                  </div>

                  <div>
                    <label
                      style={{
                        display: 'block',
                        fontSize: '11px',
                        color: 'var(--deck-text-secondary)',
                        marginBottom: '4px',
                      }}
                    >
                      {t.field_logo_art}
                    </label>
                    <input
                      type="text"
                      className="deck-input focusable mono-cell"
                      style={{ width: '100%', fontSize: '11.5px' }}
                      placeholder="Caminho do logo .png com transparência"
                      value={editingGame.art?.logo || ''}
                      onChange={(e) =>
                        setEditingGame({
                          ...editingGame,
                          art: { ...editingGame.art, logo: e.target.value },
                        })
                      }
                    />
                  </div>

                  <div>
                    <label
                      style={{
                        display: 'block',
                        fontSize: '11px',
                        color: 'var(--deck-text-secondary)',
                        marginBottom: '4px',
                      }}
                    >
                      {t.field_icon_art}
                    </label>
                    <input
                      type="text"
                      className="deck-input focusable mono-cell"
                      style={{ width: '100%', fontSize: '11.5px' }}
                      placeholder="Caminho do ícone .ico ou .png"
                      value={editingGame.art?.icon || ''}
                      onChange={(e) =>
                        setEditingGame({
                          ...editingGame,
                          art: { ...editingGame.art, icon: e.target.value },
                        })
                      }
                    />
                  </div>
                </div>
              </div>

              <div
                className="modal-footer"
                style={{
                  marginTop: '16px',
                  display: 'flex',
                  justifyContent: 'flex-end',
                  gap: '8px',
                }}
              >
                <button
                  type="button"
                  className="deck-btn focusable"
                  onClick={() => setEditGameModalOpen(false)}
                >
                  {t.btn_cancel}
                </button>
                <button
                  type="submit"
                  className="deck-btn deck-btn-primary focusable"
                >
                  {t.btn_save_game_changes}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </section>
  );
}
