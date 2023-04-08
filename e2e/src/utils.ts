import { execa } from 'execa';
import path from 'path';
import ini from 'ini';
import fs from 'fs-extra';

const DEBUG_BUILD_CMD = 'cargo build --features e2e_test';
const EXECUTABLE = '../target/debug/mfaws';
const FIXTURES = path.join(process.cwd(), 'fixtures');

const tempDir = path.join(process.cwd(), 'tmp');

function cleanup() {
  fs.emptyDirSync(tempDir);
}

export function setupDir() {
  const credsPath = path.join(tempDir, 'credentials');
  fs.copySync(FIXTURES, tempDir);
  return { cleanup, credsPath };
}

export async function buildBin() {
  return execa(DEBUG_BUILD_CMD, { all: true });
}

export function runBin(...args: string[]) {
  return execa(EXECUTABLE, args, { all: true });
}

export function iniToJSON(iniFilePath: string) {
  return ini.parse(fs.readFileSync(iniFilePath, 'utf-8'));
}
