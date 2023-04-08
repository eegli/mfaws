import fs from 'fs-extra';
import path from 'path';
import test from 'ava';
import { buildBin, FIXTURES, iniToJSON, runBin } from './utils.js';

test.before(async () => {
  const shouldBuild = process.argv.includes('--build');
  if (shouldBuild) await buildBin();
});

type Context = {
  tempDir: string;
  credsPath: string;
};

test.beforeEach(t => {
  const tempDir = fs.mkdtempSync(path.join('tmp', 't'));
  const credsPath = path.join(process.cwd(), tempDir, 'credentials');
  fs.copySync(FIXTURES, tempDir);
  t.context = <Context>{
    credsPath,
    tempDir,
  };
});

test.after(() => {
  fs.emptyDirSync('tmp');
});

test('session-token', async t => {
  const { credsPath } = t.context as Context;
  const childProcess = runBin('session-token', '--credentials-path', credsPath);

  childProcess.stdin?.write('111111');
  childProcess.stdin?.end();

  const { stdout } = await childProcess;
  t.regex(stdout, /Successfully added short-term credentials/);
  t.snapshot(iniToJSON(credsPath));
});

test('assume-role', async t => {
  const { credsPath } = t.context as Context;
  const childProcess = runBin(
    'assume-role',
    '--role-arn',
    'arn:aws:iam::41283920240:role/my-role',
    '--credentials-path',
    credsPath
  );

  childProcess.stdin?.write('111111');
  childProcess.stdin?.end();

  await childProcess;
  const { stdout } = await childProcess;
  t.regex(stdout, /Successfully added short-term credentials/);
  t.snapshot(iniToJSON(credsPath));
});

test('invalid credentials', async t => {
  const { credsPath } = t.context as Context;
  const childProcess = runBin(
    'session-token',
    '--profile',
    'notexists',
    '--credentials-path',
    credsPath
  );

  childProcess.stdin?.write('111111');
  childProcess.stdin?.end();

  const { stderr } = await childProcess;
  t.regex(stderr, /Profile "notexists" not found/);
});
