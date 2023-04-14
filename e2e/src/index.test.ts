import test from 'ava';
import { ExecaError } from 'execa';
import { buildBin, setupDir, iniToJSON, runBin } from './utils.js';

test.before(async () => {
  const shouldBuild = process.argv.includes('--build');
  if (shouldBuild) await buildBin();
});

test.serial('session-token with default profile', async t => {
  const { credsPath, cleanup } = setupDir();
  const childProcess = runBin('session-token', '--credentials-path', credsPath);

  childProcess.stdin?.write('111111');
  childProcess.stdin?.end();

  const { stdout } = await childProcess;
  t.regex(stdout, /Successfully added short-term credentials/);
  t.snapshot(iniToJSON(credsPath));
  cleanup();
});

test.serial('assume-role with custom name', async t => {
  const { credsPath, cleanup } = setupDir();
  const childProcess = runBin(
    'assume-role',
    '--role-arn',
    'arn:aws:iam::41283920240:role/my-role',
    '--role-session-name',
    'temp',
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

test.serial('with specific profile', async t => {
  const { credsPath, cleanup } = setupDir();
  const childProcess = runBin(
    'session-token',
    '--profile',
    'dev',
    '--device',
    'arn:aws:iam::123456789012:mfa/username',
    '--credentials-path',
    credsPath
  );

  childProcess.stdin?.write('111111');
  childProcess.stdin?.end();

  const { stdout } = await childProcess;
  t.regex(stdout, /Successfully added short-term credentials "dev-short-term"/);
  cleanup();
});

test.serial('without mfa device', async t => {
  const { credsPath, cleanup } = setupDir();
  const { stderr } = await runBin(
    'session-token',
    '--profile',
    'dev',
    '--credentials-path',
    credsPath
  );

  t.regex(stderr, /No MFA device found for "dev"/);
  cleanup();
});

test.serial('with invalid profile', async t => {
  const { credsPath, cleanup } = setupDir();
  const { stderr } = await runBin(
    'session-token',
    '--profile',
    'notexists',
    '--credentials-path',
    credsPath
  );

  t.regex(stderr, /Profile "notexists" not found/);
  cleanup();
});

test.serial('with invalid short-term suffix', async t => {
  const { credsPath, cleanup } = setupDir();
  const { stderr } = await runBin(
    'session-token',
    '--profile',
    'dev-short-term',
    '--credentials-path',
    credsPath
  );

  t.regex(stderr, /Profile name cannot end with the short-term suffix/);
  cleanup();
});

test.serial('with invalid credentials', async t => {
  const { cleanup } = setupDir();
  const { stderr } = await runBin(
    'session-token',
    '--credentials-path',
    'doesnotexist'
  );

  t.regex(stderr, /Failed to load credentials file/);

  cleanup();
});
