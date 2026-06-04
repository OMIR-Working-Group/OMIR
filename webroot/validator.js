/* ============================================================================
   OMIR R1 — browser reference validator (engine)
   A pure-JS implementation of the R1 conformance rules described in the spec
   and Guides: structural validity, required fields, UnitInterval ranges,
   additionalProperties:false, closed-world reference integrity, and the R1
   version marker. No network — everything runs locally in the page.
   Exposes window.OMIR.validate(text|object) and window.OMIR.EXAMPLE.
   ========================================================================== */
(function () {
  'use strict';

  // ---- R1 schema summary (field allow-lists + required + ranges) ----------
  const KNOWN = {
    Bundle:       { req: ['resourceType', 'omirVersion', 'entry'],
                    fields: ['resourceType', 'omirVersion', 'entry', 'id', 'meta', 'extension'] },
    MemoryRecord: { req: ['resourceType', 'id', 'content'],
                    fields: ['resourceType', 'id', 'content', 'createdAt', 'eventTime', 'kind',
                             'experienceType', 'importance', 'entityRefs', 'parentId', 'meta', 'extension'] },
    Entity:       { req: ['resourceType', 'id', 'name'],
                    fields: ['resourceType', 'id', 'name', 'labels', 'salience', 'meta', 'extension'] },
    Relationship: { req: ['resourceType', 'id', 'from', 'to'],
                    fields: ['resourceType', 'id', 'from', 'to', 'type', 'strength', 'sourceEpisode', 'meta', 'extension'] },
    Episode:      { req: ['resourceType', 'id'],
                    fields: ['resourceType', 'id', 'title', 'content', 'startTime', 'endTime', 'entityRefs', 'meta', 'extension'] }
  };
  const UNIT = { MemoryRecord: ['importance'], Entity: ['salience'], Relationship: ['strength'] };
  const ENUMS = {
    kind: ['fact', 'preference', 'event', 'learning', 'task', 'belief', 'observation'],
    experienceType: ['decision', 'conversation', 'action', 'reflection', 'perception']
  };

  const EXAMPLE = {
    resourceType: 'Bundle',
    omirVersion: 'R1',
    entry: [
      { resourceType: 'Entity', id: 'rust', name: 'Rust programming',
        labels: ['technology', 'skill'], salience: 0.8 },
      { resourceType: 'Entity', id: 'jane', name: 'Jane', labels: ['person'], salience: 0.6 },
      { resourceType: 'MemoryRecord', id: 'm1',
        content: 'The team standardized on Rust for the reference validator.',
        createdAt: '2026-06-01T09:00:00Z', kind: 'learning', experienceType: 'decision',
        importance: 0.8, entityRefs: [{ ref: 'Entity/rust' }, { ref: 'Entity/jane' }] },
      { resourceType: 'Relationship', id: 'r1', from: 'Entity/jane', to: 'Entity/rust',
        type: 'prefers', strength: 0.7 }
    ]
  };

  function isObj(v) { return v && typeof v === 'object' && !Array.isArray(v); }
  function add(out, level, path, msg) { out.findings.push({ level: level, path: path, msg: msg }); }

  function validate(input) {
    const out = { ok: false, findings: [], counts: {}, nodes: [], edges: [], parsed: null };

    // ---- parse -------------------------------------------------------------
    let doc = input;
    if (typeof input === 'string') {
      try { doc = JSON.parse(input); }
      catch (e) {
        add(out, 'error', '(document)', 'Invalid JSON: ' + e.message);
        return out;
      }
    }
    out.parsed = doc;

    // ---- envelope ----------------------------------------------------------
    if (!isObj(doc)) { add(out, 'error', '(document)', 'Top level must be a JSON object (a Bundle).'); return out; }
    if (doc.resourceType !== 'Bundle')
      add(out, 'error', 'resourceType', 'Top-level resourceType must be "Bundle" (got ' + JSON.stringify(doc.resourceType) + ').');
    if (doc.omirVersion === undefined)
      add(out, 'error', 'omirVersion', 'Missing version marker: Bundle.omirVersion must be "R1".');
    else if (doc.omirVersion !== 'R1')
      add(out, 'error', 'omirVersion', 'Unsupported version ' + JSON.stringify(doc.omirVersion) + ' — this validator checks R1.');
    checkUnknown(out, 'Bundle', doc, '');

    if (!Array.isArray(doc.entry)) {
      add(out, 'error', 'entry', 'Bundle.entry must be an array of resources.');
      return finish(out);
    }
    if (doc.entry.length === 0)
      add(out, 'warn', 'entry', 'Bundle has no entries — valid but empty.');

    // ---- per-entry structural pass + id collection -------------------------
    const ids = { MemoryRecord: new Set(), Entity: new Set(), Relationship: new Set(), Episode: new Set() };
    const counts = {};
    doc.entry.forEach(function (res, i) {
      const at = 'entry[' + i + ']';
      if (!isObj(res)) { add(out, 'error', at, 'Each entry must be a JSON object.'); return; }
      const rt = res.resourceType;
      if (!KNOWN[rt]) { add(out, 'error', at + '.resourceType', 'Unknown resourceType ' + JSON.stringify(rt) + '.'); return; }
      counts[rt] = (counts[rt] || 0) + 1;

      // required fields
      KNOWN[rt].req.forEach(function (f) {
        if (res[f] === undefined || res[f] === null || res[f] === '')
          add(out, 'error', at + '.' + f, 'Missing required field "' + f + '" on ' + rt + '.');
      });
      // additionalProperties:false
      checkUnknown(out, rt, res, at + '.');
      // unit intervals [0,1]
      (UNIT[rt] || []).forEach(function (f) {
        if (res[f] !== undefined) {
          const v = res[f];
          if (typeof v !== 'number' || v < 0 || v > 1)
            add(out, 'error', at + '.' + f, '"' + f + '" must be a UnitInterval in [0,1] (got ' + JSON.stringify(v) + ').');
        }
      });
      // enums (lenient → warn)
      Object.keys(ENUMS).forEach(function (f) {
        if (res[f] !== undefined && ENUMS[f].indexOf(res[f]) === -1)
          add(out, 'warn', at + '.' + f, '"' + res[f] + '" is not a known ' + f + ' value.');
      });
      // version marker on entries (optional, warn on mismatch)
      if (res.meta && res.meta.omirVersion && res.meta.omirVersion !== 'R1')
        add(out, 'warn', at + '.meta.omirVersion', 'Entry version marker is not "R1".');

      // collect id
      if (res.id !== undefined && ids[rt]) {
        if (ids[rt].has(res.id))
          add(out, 'error', at + '.id', 'Duplicate ' + rt + ' id "' + res.id + '".');
        ids[rt].add(res.id);
      }
    });
    out.counts = counts;

    // ---- reference integrity (closed-world) --------------------------------
    let refTotal = 0, refOk = 0;
    function resolve(out2, at, refStr, expectType) {
      refTotal++;
      if (typeof refStr !== 'string' || refStr.indexOf('/') === -1) {
        add(out, 'error', at, 'Reference must be of the form "' + expectType + '/<id>" (got ' + JSON.stringify(refStr) + ').');
        return;
      }
      const parts = refStr.split('/');
      const type = parts[0], id = parts.slice(1).join('/');
      if (type !== expectType)
        add(out, 'error', at, 'Reference must target ' + expectType + ' (got "' + type + '").');
      else if (!ids[type] || !ids[type].has(id))
        add(out, 'error', at, 'Dangling reference "' + refStr + '" — no ' + expectType + ' with id "' + id + '" in this Bundle.');
      else refOk++;
    }

    doc.entry.forEach(function (res, i) {
      if (!isObj(res) || !KNOWN[res.resourceType]) return;
      const at = 'entry[' + i + ']';
      const rt = res.resourceType;
      if ((rt === 'MemoryRecord' || rt === 'Episode') && Array.isArray(res.entityRefs))
        res.entityRefs.forEach(function (er, j) {
          resolve(out, at + '.entityRefs[' + j + '].ref', er && er.ref, 'Entity');
        });
      if (rt === 'Relationship') {
        if (res.from !== undefined) resolve(out, at + '.from', res.from, 'Entity');
        if (res.to !== undefined) resolve(out, at + '.to', res.to, 'Entity');
        if (res.sourceEpisode !== undefined) resolve(out, at + '.sourceEpisode', res.sourceEpisode, 'Episode');
      }
      if (rt === 'MemoryRecord' && res.parentId !== undefined) {
        refTotal++;
        if (!ids.MemoryRecord.has(res.parentId))
          add(out, 'error', at + '.parentId', 'parentId "' + res.parentId + '" does not resolve to a MemoryRecord in this Bundle.');
        else refOk++;
      }
    });
    out.refStats = { total: refTotal, resolved: refOk };

    // ---- build graph data --------------------------------------------------
    doc.entry.forEach(function (res) {
      if (!isObj(res)) return;
      if (res.resourceType === 'Entity')
        out.nodes.push({ id: res.id, name: res.name || res.id, salience: typeof res.salience === 'number' ? res.salience : 0.5 });
      if (res.resourceType === 'Relationship' && typeof res.from === 'string' && typeof res.to === 'string')
        out.edges.push({ from: res.from.split('/').slice(1).join('/'), to: res.to.split('/').slice(1).join('/'),
          strength: typeof res.strength === 'number' ? res.strength : 0.5, type: res.type || '' });
    });

    return finish(out);
  }

  function checkUnknown(out, rt, obj, prefix) {
    const allowed = KNOWN[rt].fields;
    Object.keys(obj).forEach(function (k) {
      if (allowed.indexOf(k) === -1)
        add(out, 'error', prefix + k,
          'Unknown field "' + k + '" on ' + rt + ' — the core sets additionalProperties:false. Put bespoke data in extension[].');
    });
  }

  function finish(out) {
    const errs = out.findings.filter(function (f) { return f.level === 'error'; });
    out.ok = errs.length === 0;
    return out;
  }

  window.OMIR = { validate: validate, EXAMPLE: EXAMPLE, KNOWN: KNOWN };
})();
