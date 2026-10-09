#!/usr/bin/env python3
"""Fresh, seeded Rust/Lean execution comparisons; these are tests, not refinement proofs."""
import argparse
import hashlib
import json
from pathlib import Path
import random
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
OPS = ('add', 'sub', 'mul', 'and', 'or', 'xor', 'shl', 'lshr')


def run(command, **kwargs):
    return subprocess.check_output(command, text=True, timeout=120, **kwargs)


def expression(rng, depth):
    if depth == 0:
        return rng.choice(('x', 'y'))
    return (rng.choice(OPS), expression(rng, depth - 1), expression(rng, depth - 1))


def rust_expr(expr):
    if isinstance(expr, str):
        return expr
    return f'{expr[0]}({rust_expr(expr[1])}, {rust_expr(expr[2])})'


def lean_expr(expr):
    if isinstance(expr, str):
        return f'(.var "{expr}")'
    return f'(.binop .{expr[0]} {lean_expr(expr[1])} {lean_expr(expr[2])})'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', required=True, type=Path)
    parser.add_argument('--seed', type=int, default=20261008)
    args = parser.parse_args()
    rng = random.Random(args.seed)
    vectors = []
    for width in (8, 16, 32, 64):
        for op in OPS:
            vectors.append((width, (op, 'x', 'y'), (1 << width) - 1, width))
        for _ in range(32):
            vectors.append((width, expression(rng, 2), rng.getrandbits(width), rng.getrandbits(width)))
    for op in ('shl', 'lshr'):
        vectors.append((64, (op, 'x', 'y'), 1, 1 << 32))
    with tempfile.TemporaryDirectory(prefix='certiforge-differential-') as temporary:
        folder = Path(temporary)
        lean = ['import CertiForge.Semantics', 'open CertiForge']
        rust_values = []
        cases = []
        for index, (width, expr, x, y) in enumerate(vectors):
            source = f'fn replay(x: u{width}, y: u{width}) -> u{width} {{ {rust_expr(expr)} }}\n'
            file = folder / f'case-{index}.certir'
            file.write_text(source)
            output = run([str(ROOT / 'target/debug/certiforge'), 'eval', str(file), '--inputs', f'{x},{y}'])
            match = re.fullmatch(r'BitVec \{ width: U\d+, bits: (\d+) \}\s*', output)
            if not match:
                raise RuntimeError('Rust output is not a supported bitvector result')
            rust_values.append(int(match[1]))
            lean.append(f'def case{index} : Program := {{ params := [("x", .bitvec .w{width}), ("y", .bitvec .w{width})], body := {lean_expr(expr)}, retTy := .bitvec .w{width} }}')
            lean.append(f'#eval match eval case{index} [.bitvec .w{width} (BitVec.ofNat {width} {x}), .bitvec .w{width} (BitVec.ofNat {width} {y})] with | some (.bitvec _ v) => v.toNat | _ => 99999999999999999999999999')
            cases.append({'width': width, 'source': source, 'inputs': [x, y], 'rust_value': int(match[1])})
        lean_file = folder / 'Replay.lean'
        lean_file.write_text('\n'.join(lean)+'\n')
        output = run(['lake', 'env', 'lean', str(lean_file)], cwd=ROOT / 'formal')
        lines = output.splitlines()
        if len(lines) != len(vectors) or not all(re.fullmatch(r'\d+', line) for line in lines):
            raise RuntimeError('Lean output incomplete or contains diagnostics')
        lean_values = list(map(int, lines))
        mismatches = []
        for i, case in enumerate(cases):
            case['lean_value'] = lean_values[i]
            if rust_values[i] != lean_values[i]:
                mismatches.append(i)
        report = {'format': 'certiforge-fresh-differential-replay-v1', 'seed': args.seed,
                  'source_commit': run(['git', 'rev-parse', 'HEAD'], cwd=ROOT).strip(),
                  'source_dirty': bool(run(['git', 'status', '--porcelain'], cwd=ROOT).strip()),
                  'checker_sha256': hashlib.sha256((ROOT/'target/debug/certiforge').read_bytes()).hexdigest(),
                  'lean_toolchain': (ROOT/'formal/lean-toolchain').read_text().strip(),
                  'cases': cases, 'passed': len(cases)-len(mismatches), 'mismatches': mismatches,
                  'refinement_proved': False, 'coverage': 'Eight binary operators, four widths, edge cases and depth-two random ASTs. Unary, comparison and select operations not covered.'}
        with args.output.open('x') as file:
            json.dump(report, file, sort_keys=True, indent=2)
            file.write('\n')
        print(json.dumps({'passed': report['passed'], 'total': len(cases), 'mismatches': mismatches, 'refinement_proved': False}))
        if mismatches:
            raise SystemExit(1)


if __name__ == '__main__':
    main()
