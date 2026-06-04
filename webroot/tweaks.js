/* ============================================================================
   OMIR — shared Tweaks panel
   A self-contained design-review panel available on every page. Persists to
   localStorage and applies on load, so a choice carries across the whole site.
   Controls: accent colour, font pairing, hero layout (homepage only).
   ========================================================================== */
(function () {
  'use strict';
  var KEY = 'omir-tweaks';
  var root = document.documentElement;
  var body = document.body;

  var ACCENTS = [
    { id: 'cobalt', label: 'Cobalt', hex: '#2f55d4' },
    { id: 'indigo', label: 'Indigo', hex: '#4f46e5' },
    { id: 'teal',   label: 'Teal',   hex: '#0f7d6b' },
    { id: 'clay',   label: 'Clay',   hex: '#c2552f' }
  ];
  var FONTS = [
    { id: 'archivo', label: 'Archivo', display: 'Archivo', body: 'IBM Plex Sans' },
    { id: 'grotesk', label: 'Grotesk', display: 'Space Grotesk', body: 'IBM Plex Sans' },
    { id: 'plex',    label: 'Plex',    display: 'IBM Plex Sans', body: 'IBM Plex Sans' }
  ];
  var LAYOUTS = [
    { id: 'split',   label: 'Split' },
    { id: 'stacked', label: 'Stacked' }
  ];

  var state = load();
  apply(state);

  function load() {
    var d = { accent: 'cobalt', font: 'archivo', layout: 'split' };
    try { var s = JSON.parse(localStorage.getItem(KEY)); if (s) Object.assign(d, s); } catch (e) {}
    return d;
  }
  function save() { try { localStorage.setItem(KEY, JSON.stringify(state)); } catch (e) {} }

  function apply(s) {
    var a = ACCENTS.filter(function (x) { return x.id === s.accent; })[0] || ACCENTS[0];
    root.style.setProperty('--accent', a.hex);
    var f = FONTS.filter(function (x) { return x.id === s.font; })[0] || FONTS[0];
    root.style.setProperty('--font-display', "'" + f.display + "'");
    root.style.setProperty('--font-body', "'" + f.body + "'");
    if (body) body.classList.toggle('hero-stacked', s.layout === 'stacked');
  }

  // ---- panel UI -------------------------------------------------------------
  function build() {
    if (document.getElementById('omir-tweaks-fab')) return;
    var isHome = body.classList.contains('home');

    var fab = document.createElement('button');
    fab.id = 'omir-tweaks-fab';
    fab.setAttribute('aria-label', 'Open design tweaks');
    fab.innerHTML = svgGear();

    var panel = document.createElement('div');
    panel.id = 'omir-tweaks-panel';
    panel.hidden = true;
    panel.innerHTML =
      '<div class="tw-head"><span class="tw-title">Tweaks</span>' +
        '<button class="tw-x" aria-label="Close">\u00d7</button></div>' +
      group('Accent', swatches()) +
      group('Type pairing', radios('font', FONTS)) +
      (isHome ? group('Hero layout', radios('layout', LAYOUTS)) : '') +
      '<button class="tw-reset">Reset to defaults</button>';

    var style = document.createElement('style');
    style.textContent = css();
    document.head.appendChild(style);
    document.body.appendChild(fab);
    document.body.appendChild(panel);

    fab.addEventListener('click', function () { panel.hidden = !panel.hidden; sync(); });
    panel.querySelector('.tw-x').addEventListener('click', function () { panel.hidden = true; });
    panel.querySelector('.tw-reset').addEventListener('click', function () {
      state = { accent: 'cobalt', font: 'archivo', layout: 'split' };
      apply(state); save(); sync();
    });

    panel.addEventListener('click', function (e) {
      var sw = e.target.closest('[data-accent]');
      if (sw) { state.accent = sw.getAttribute('data-accent'); apply(state); save(); sync(); }
      var rb = e.target.closest('[data-group]');
      if (rb) { state[rb.getAttribute('data-group')] = rb.getAttribute('data-val'); apply(state); save(); sync(); }
    });

    sync();

    function sync() {
      [].forEach.call(panel.querySelectorAll('[data-accent]'), function (el) {
        el.classList.toggle('on', el.getAttribute('data-accent') === state.accent);
      });
      [].forEach.call(panel.querySelectorAll('[data-group]'), function (el) {
        el.classList.toggle('on', state[el.getAttribute('data-group')] === el.getAttribute('data-val'));
      });
    }
  }

  function group(title, inner) {
    return '<div class="tw-group"><div class="tw-label">' + title + '</div>' + inner + '</div>';
  }
  function swatches() {
    return '<div class="tw-swatches">' + ACCENTS.map(function (a) {
      return '<button class="tw-sw" data-accent="' + a.id + '" title="' + a.label + '" ' +
        'style="background:' + a.hex + '"></button>';
    }).join('') + '</div>';
  }
  function radios(grp, items) {
    return '<div class="tw-seg">' + items.map(function (it) {
      return '<button class="tw-opt" data-group="' + grp + '" data-val="' + it.id + '">' + it.label + '</button>';
    }).join('') + '</div>';
  }

  function svgGear() {
    return '<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" ' +
      'stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">' +
      '<path d="M4 7h10M18 7h2M4 12h2M10 12h10M4 17h7M15 17h5"/>' +
      '<circle cx="16" cy="7" r="2.2" fill="#fff"/><circle cx="8" cy="12" r="2.2" fill="#fff"/>' +
      '<circle cx="13" cy="17" r="2.2" fill="#fff"/></svg>';
  }

  function css() {
    return [
      '#omir-tweaks-fab{position:fixed;right:20px;bottom:20px;z-index:9998;width:46px;height:46px;',
      'border-radius:999px;border:1px solid #e6e8ec;background:#0e1116;color:#fff;cursor:pointer;',
      'display:flex;align-items:center;justify-content:center;box-shadow:0 6px 22px -8px rgba(14,17,22,.5);',
      'transition:transform .15s}',
      '#omir-tweaks-fab:hover{transform:translateY(-1px)}',
      '#omir-tweaks-panel{position:fixed;right:20px;bottom:78px;z-index:9999;width:248px;background:#fff;',
      'border:1px solid #e6e8ec;border-radius:14px;padding:14px;font-family:var(--font-body),system-ui,sans-serif;',
      'box-shadow:0 18px 50px -18px rgba(14,17,22,.4)}',
      '#omir-tweaks-panel[hidden]{display:none}',
      '.tw-head{display:flex;align-items:center;justify-content:space-between;margin-bottom:12px}',
      '.tw-title{font-family:var(--font-display),sans-serif;font-weight:800;font-size:.95rem;letter-spacing:-.02em}',
      '.tw-x{background:none;border:none;font-size:1.2rem;line-height:1;color:#8a8f96;cursor:pointer;padding:2px 6px}',
      '.tw-group{margin:12px 0}',
      '.tw-label{font-family:var(--font-mono),monospace;font-size:.62rem;letter-spacing:.12em;',
      'text-transform:uppercase;color:#8a8f96;margin-bottom:8px}',
      '.tw-swatches{display:flex;gap:8px}',
      '.tw-sw{width:30px;height:30px;border-radius:8px;border:2px solid transparent;cursor:pointer;',
      'outline:1px solid rgba(0,0,0,.08);outline-offset:-1px}',
      '.tw-sw.on{border-color:#0e1116}',
      '.tw-seg{display:flex;gap:6px;flex-wrap:wrap}',
      '.tw-opt{flex:1;min-width:62px;padding:7px 8px;border:1px solid #e6e8ec;border-radius:8px;background:#fff;',
      'cursor:pointer;font-size:.74rem;font-weight:500;color:#5b646e;white-space:nowrap}',
      '.tw-opt:hover{border-color:#cfd5db}',
      '.tw-opt.on{background:#0e1116;color:#fff;border-color:#0e1116}',
      '.tw-reset{width:100%;margin-top:6px;padding:8px;border:1px solid #e6e8ec;border-radius:8px;',
      'background:#fafbfc;cursor:pointer;font-size:.74rem;color:#5b646e;font-family:var(--font-mono),monospace;',
      'letter-spacing:.04em}',
      '.tw-reset:hover{border-color:#cfd5db}',
      '@media print{#omir-tweaks-fab,#omir-tweaks-panel{display:none!important}}'
    ].join('');
  }

  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', build);
  else build();
})();
