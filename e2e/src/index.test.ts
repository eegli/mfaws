import { execa } from 'execa';
import fs from 'fs';
import path from 'path';
import test from 'ava';

const EXE = '../target/debug/mfaws';
const CREDENTIALS = path.join(process.cwd(), 'fixtures', 'credentials');

const DEBUG_BUILD_CMD = 'cargo build --features e2e_test';

test('session-token', async t => {
  const TEMP_DIR = path.join(process.cwd(), fs.mkdtempSync('test'));
  const TEMP_CREDS = path.join(TEMP_DIR, 'credentials');

  fs.copyFileSync(CREDENTIALS, TEMP_CREDS);

  const childProcess = execa(
    EXE,
    ['session-token', '--credentials-path', TEMP_CREDS],
    { all: true }
  );

  // childProcess.all?.pipe(process.stdout);
  childProcess.stdin?.write('111111');
  childProcess.stdin?.end();
  await childProcess;

  t.snapshot(fs.readFileSync(TEMP_CREDS, 'utf-8'));
  fs.rmSync(TEMP_DIR, { recursive: true, force: true });
  t.pass();
});

test('assume-role', async t => {
  const TEMP_DIR = path.join(process.cwd(), fs.mkdtempSync('test'));
  const TEMP_CREDS = path.join(TEMP_DIR, 'credentials');

  fs.copyFileSync(CREDENTIALS, TEMP_CREDS);

  const childProcess = execa(
    EXE,
    ['assume-role', '--role-arn', 'arn/role', '--credentials-path', TEMP_CREDS],
    { all: true }
  );

  // childProcess.all?.pipe(process.stdout);
  childProcess.stdin?.write('111111');
  childProcess.stdin?.end();
  await childProcess;

  t.snapshot(fs.readFileSync(TEMP_CREDS, 'utf-8'));
  fs.rmSync(TEMP_DIR, { recursive: true, force: true });
  t.pass();
});
