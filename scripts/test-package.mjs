import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';

const root = process.cwd();
const temporary = mkdtempSync(resolve('.package-test-'));
const npm = process.platform === 'win32' ? 'npm.cmd' : 'npm';
const run = (command, args, cwd = temporary) =>
  execFileSync(command, args, {
    cwd,
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'inherit'],
  });
try {
  const [pack] = JSON.parse(
    run(
      npm,
      ['pack', '--ignore-scripts', '--json', '--pack-destination', temporary],
      root,
    ),
  );
  assert(pack.files.some(({ path }) => path === 'LICENSE'));
  assert(
    pack.files.every(
      ({ path }) => !/^(src|tests|scripts|node_modules|\.github)\//.test(path),
    ),
  );
  writeFileSync(resolve(temporary, 'package.json'), '{"private":true}');
  run(npm, [
    'install',
    '--ignore-scripts',
    '--no-audit',
    '--no-fund',
    resolve(temporary, pack.filename),
  ]);
  const consumer = `
    import assert from 'node:assert/strict';
    import { SECClient } from 'sec-data-fetcher';
    const client = new SECClient({ userAgent: 'Package test test@example.com' });
    assert.deepEqual(client.extractTablesFromContent('<table><tr><td>42</td></tr></table>'), [[['42']]]);
    assert.deepEqual(client.getObjectFromString('<filing><value>42</value></filing>'), { filing: { value: 42 } });
  `;
  writeFileSync(resolve(temporary, 'consumer.mjs'), consumer);
  writeFileSync(
    resolve(temporary, 'consumer.cjs'),
    consumer
      .replace(
        "import assert from 'node:assert/strict';",
        "const assert = require('node:assert/strict');",
      )
      .replace(
        "import { SECClient } from 'sec-data-fetcher';",
        "const { SECClient } = require('sec-data-fetcher');",
      ),
  );
  run(process.execPath, ['consumer.mjs']);
  run(process.execPath, ['consumer.cjs']);
  const types = `import { SECClient, type CompanyFacts, type CompanySubmissions, type Filing } from 'sec-data-fetcher';
    const client = new SECClient({ userAgent: 'Types test@example.com' });
    const facts: Promise<CompanyFacts> = client.getCompanyFacts('320193');
    const company: Promise<CompanySubmissions> = client.getCompanyData('320193');
    const reports: Promise<Filing[]> = client.getReports('320193');`;
  for (const extension of ['mts', 'cts']) {
    const file = `consumer.${extension}`;
    writeFileSync(resolve(temporary, file), types);
    run(process.execPath, [
      resolve(root, 'node_modules/typescript/bin/tsc'),
      '--noEmit',
      '--strict',
      '--module',
      'Node16',
      '--target',
      'ES2022',
      file,
    ]);
  }
  const installed = JSON.parse(
    readFileSync(
      resolve(temporary, 'node_modules/sec-data-fetcher/package.json'),
    ),
  );
  console.log(
    `Package ${installed.version}: CommonJS, ESM and TypeScript consumers passed (${pack.size} bytes packed).`,
  );
} finally {
  rmSync(temporary, { recursive: true, force: true });
}
