import { execa } from 'execa';
import fs from 'fs';
import path from 'path';

const EXE = '../target/debug/mfaws';
const FIXTURES = path.join(process.cwd(), 'fixtures', 'credentials');

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
  childProcess.stdin.write('121212');
  childProcess.stdin.end();
  await childProcess;
} catch (err) {
  console.error(err);
} finally {
  tearDown();
}
