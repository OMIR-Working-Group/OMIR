/* ============================================================================
   OMIR — Markdown export
   Converts rendered page content to Markdown and triggers a download. Used for
   ".md downloads of the spec": each page can export itself, and the
   Specification page can bundle the whole R1 spec into one file by fetching the
   built spec/R1/* pages (same-origin) and concatenating their Markdown.
   Exposes window.OMIRMarkdown.{htmlToMarkdown, download, exportPage, exportSpec}.
   ========================================================================== */
(function () {
  'use strict';

  function txt(node) { return (node.textContent || '').replace(/\s+/g, ' '); }

  // inline → markdown (handles a/strong/em/code and nested text)
  function inline(node) {
    let out = '';
    node.childNodes.forEach(function (n) {
      if (n.nodeType === 3) { out += n.textContent.replace(/\s+/g, ' '); return; }
      if (n.nodeType !== 1) return;
      const tag = n.tagName.toLowerCase();
      if (tag === 'a') {
        const href = n.getAttribute('href') || '';
        out += '[' + inline(n).trim() + '](' + href + ')';
      } else if (tag === 'strong' || tag === 'b') {
        out += '**' + inline(n).trim() + '**';
      } else if (tag === 'em' || tag === 'i') {
        out += '*' + inline(n).trim() + '*';
      } else if (tag === 'code') {
        out += '`' + txt(n).trim() + '`';
      } else if (tag === 'br') {
        out += '  \n';
      } else {
        out += inline(n);
      }
    });
    return out;
  }

  function listItems(listEl, ordered, depth) {
    let out = '', i = 1;
    listEl.childNodes.forEach(function (li) {
      if (li.nodeType !== 1 || li.tagName.toLowerCase() !== 'li') return;
      const bullet = ordered ? (i++ + '. ') : '- ';
      const pad = '  '.repeat(depth);
      // text of the li excluding nested lists
      const clone = li.cloneNode(true);
      const nested = [];
      clone.querySelectorAll(':scope > ul, :scope > ol').forEach(function (n) { nested.push(n); n.remove(); });
      out += pad + bullet + inline(clone).trim() + '\n';
      li.querySelectorAll(':scope > ul, :scope > ol').forEach(function (n) {
        out += listItems(n, n.tagName.toLowerCase() === 'ol', depth + 1);
      });
    });
    return out;
  }

  function table(tbl) {
    const rows = [].slice.call(tbl.querySelectorAll('tr'));
    if (!rows.length) return '';
    let out = '', headerDone = false;
    rows.forEach(function (tr, ri) {
      const cells = [].slice.call(tr.children).map(function (c) { return inline(c).trim().replace(/\|/g, '\\|'); });
      out += '| ' + cells.join(' | ') + ' |\n';
      const isHeader = tr.querySelector('th');
      if ((isHeader || ri === 0) && !headerDone) {
        out += '| ' + cells.map(function () { return '---'; }).join(' | ') + ' |\n';
        headerDone = true;
      }
    });
    return out + '\n';
  }

  function htmlToMarkdown(root) {
    let md = '';
    root.childNodes.forEach(function (node) {
      if (node.nodeType === 3) { const t = node.textContent.trim(); if (t) md += t + '\n\n'; return; }
      if (node.nodeType !== 1) return;
      const tag = node.tagName.toLowerCase();
      if (tag === 'button' || tag === 'script' || tag === 'style' || tag === 'svg' || tag === 'textarea') return;
      if (node.hasAttribute('data-no-export')) return;
      if (node.classList && (node.classList.contains('dl-bar') || node.classList.contains('pg-actions'))) return;
      if (/^h[1-6]$/.test(tag)) {
        md += '\n' + '#'.repeat(+tag[1]) + ' ' + inline(node).trim() + '\n\n';
      } else if (tag === 'p') {
        const t = inline(node).trim(); if (t) md += t + '\n\n';
      } else if (tag === 'ul' || tag === 'ol') {
        md += listItems(node, tag === 'ol', 0) + '\n';
      } else if (tag === 'pre') {
        md += '```\n' + (node.textContent || '').replace(/\n$/, '') + '\n```\n\n';
      } else if (tag === 'blockquote') {
        md += inline(node).trim().split('\n').map(function (l) { return '> ' + l; }).join('\n') + '\n\n';
      } else if (tag === 'table') {
        md += table(node);
      } else if (tag === 'hr') {
        md += '---\n\n';
      } else if (['div', 'section', 'article', 'header', 'main'].indexOf(tag) !== -1) {
        md += htmlToMarkdown(node);             // recurse into containers
      } else {
        const t = inline(node).trim(); if (t) md += t + '\n\n';
      }
    });
    return md;
  }

  function contentRoot(doc) {
    return doc.querySelector('main') || doc.querySelector('.content')
      || doc.querySelector('article') || doc.body;
  }

  function download(filename, md) {
    const blob = new Blob([md], { type: 'text/markdown;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url; a.download = filename;
    document.body.appendChild(a); a.click(); a.remove();
    setTimeout(function () { URL.revokeObjectURL(url); }, 2000);
  }

  function exportPage(filename) {
    const md = '<!-- OMIR · exported ' + new Date().toISOString().slice(0, 10) + ' -->\n\n'
      + htmlToMarkdown(contentRoot(document)).replace(/\n{3,}/g, '\n\n').trim() + '\n';
    download(filename || (document.title.replace(/\s+/g, '-').toLowerCase() + '.md'), md);
  }

  // Bundle the whole R1 spec: follow every ./spec/R1/* link on the page,
  // fetch each, convert, concatenate. Works on the deployed site.
  async function exportSpec(btn) {
    const links = [].slice.call(document.querySelectorAll('main a[href*="spec/R1/"]'))
      .map(function (a) { return a.getAttribute('href'); });
    const seen = {}, list = [];
    links.forEach(function (h) { if (h && !seen[h]) { seen[h] = 1; list.push(h); } });
    if (!list.length) { alert('No spec/R1 links found on this page.'); return; }

    const label = btn && btn.textContent;
    if (btn) { btn.disabled = true; btn.textContent = 'Bundling…'; }
    let md = '# OMIR R1 — Specification\n\n_Exported ' + new Date().toISOString().slice(0, 10)
      + ' from the OMIR site. Source of truth: the mdBook under `spec/`._\n';
    let okCount = 0;
    for (const href of list) {
      try {
        const res = await fetch(href);
        if (!res.ok) throw new Error(res.status);
        const doc = new DOMParser().parseFromString(await res.text(), 'text/html');
        md += '\n\n---\n\n' + htmlToMarkdown(contentRoot(doc)).replace(/\n{3,}/g, '\n\n').trim() + '\n';
        okCount++;
      } catch (e) {
        md += '\n\n---\n\n> _Could not load `' + href + '` (' + e.message + ') — build the spec with `build.sh` first._\n';
      }
    }
    download('omir-R1-spec.md', md);
    if (btn) { btn.disabled = false; btn.textContent = label; }
    if (!okCount) alert('The spec/R1 pages aren’t built yet in this environment. On the deployed site (after build.sh) this produces the full Markdown.');
  }

  window.OMIRMarkdown = { htmlToMarkdown: htmlToMarkdown, download: download, exportPage: exportPage, exportSpec: exportSpec };
})();
