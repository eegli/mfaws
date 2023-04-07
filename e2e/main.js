import { execa } from 'execa';
import fs from 'fs';
import path from 'path';

const EXE = '../target/debug/mfaws';
const FIXTURES = path.join(process.cwd(), 'fixtures', 'credentials');

const DEBUG_BUILD_CMD = 'cargo build --features e2e_test';

function setup() {
  const tempDir = fs.mkdtempSync(path.join(process.cwd(), 'test-dir'));
  fs.copyFileSync(FIXTURES, path.join(tempDir, 'credentials'));
  function tearDown() {
    fs.rmSync(tempDir, { recursive: true, force: true });
  }
  return { tempDir, tearDown };
}

const { tempDir, tearDown } = setup();

const credentialsPath = `${tempDir}/credentials`;

try {
  const childProcess = execa(
    EXE,
    ['session-token', '--credentials-path', credentialsPath],
    { all: true }
  );
  childProcess.all.pipe(process.stdout);
  childProcess.stdin.write('111111');
  childProcess.stdin.end();
  await childProcess;
  console.log(fs.readFileSync(credentialsPath, 'utf-8'));
} catch (err) {
  console.error(err);
} finally {
  tearDown();
}
