import { execa } from 'execa';
import path from 'path';
import ini from 'ini';
import fs from 'fs';

const DEBUG_BUILD_CMD = 'cargo build --features e2e_test';
const EXECUTABLE = '../target/debug/mfaws';
export const FIXTURES = path.join(process.cwd(), 'fixtures');

export async function buildBin() {
  return execa(DEBUG_BUILD_CMD, { all: true });
}

export function runBin(...args: string[]) {
  return execa(EXECUTABLE, args, { all: true });
}

export function iniToJSON(iniFilePath: string) {
  return ini.parse(fs.readFileSync(iniFilePath, 'utf-8'));
}
