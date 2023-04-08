import test from 'ava';
import { buildBin, setupDir, iniToJSON, runBin } from './utils.js';

test.before(async () => {
  const shouldBuild = process.argv.includes('--build');
  if (shouldBuild) await buildBin();
});

test.serial('session-token', async t => {
  const { credsPath, cleanup } = setupDir();
  const childProcess = runBin('session-token', '--credentials-path', credsPath);

  childProcess.stdin?.write('111111');
  childProcess.stdin?.end();

  const { stdout } = await childProcess;
  t.regex(stdout, /Successfully added short-term credentials/);
  t.snapshot(iniToJSON(credsPath));
  cleanup();
});

test.serial('assume-role', async t => {
  const { credsPath, cleanup } = setupDir();
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
  cleanup();
});

test.serial('invalid credentials', async t => {
  const { credsPath, cleanup } = setupDir();
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
  cleanup();
});
