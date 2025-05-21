import json

with open('coreutils_metrics.json', 'r') as f:
    data = json.load(f)

print('\\resizebox{\\linewidth}{!}{%\n'
      '\\begin{tabular}{lrrrrrr}\n'
       '\\clineB{2-7}{2}')

print('''\\multicolumn{1}{lV{2}}{\\multirow{3}{*}{}} &
  \\multicolumn{3}{cV{2}}{Raw Pointer} &
  \\multicolumn{3}{cV{2}}{Raw Pointer}\\bigstrut[t]\\\\
\\multicolumn{1}{lV{2}}{} &
  \\multicolumn{3}{cV{2}}{Declarations ($\\nabla$)} &
  \\multicolumn{3}{cV{2}}{Deferences  ($\\nabla$)} \\bigstrut[b]\\\\ \\clineB{2-7}{2}
\\multicolumn{1}{lV{2}}{} &
  \\multicolumn{1}{rV{2}}{\\rotatebox{90}{Before~~}} &
  \\multicolumn{1}{rV{2}}{{\\rotatebox{90}{After}}} &
  \\multicolumn{1}{cV{2}}{$\\Delta\\%$} &
  \\multicolumn{1}{rV{2}}{\\rotatebox{90}{Before~~}} &
  \\multicolumn{1}{rV{2}}{{\\rotatebox{90}{After}}} &
  \\multicolumn{1}{cV{2}}{$\\Delta\\%$} \\bigstrut\\\\ \\clineB{2-7}{2}
 &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} \\bigstrut\\\\[-1.33em] \\hlineB{2}''')

for program in data.keys():
    original = data[program]['original']
    translated = data[program]['translated']

    original_decls = original['raw_pointer_decls']
    original_derefs = original['raw_pointer_derefs']

    translated_decls = translated['raw_pointer_decls']
    translated_derefs = translated['raw_pointer_derefs']

    improvement_decls = 100 * (original_decls - translated_decls) / original_decls
    improvement_derefs = 100 * (original_derefs - translated_derefs) / original_derefs

    print(f'\\multicolumn{{1}}{{V{{2}}lV{{2}}}}{{{program}}}\n'
        f'& \\multicolumn{{1}}{{rV{{2}}}}{{{original_decls}}}\n'
        f'& \\multicolumn{{1}}{{rV{{2}}}}{{{translated_decls}}}\n'
        f'& \\multicolumn{{1}}{{rV{{2}}}}{{\\cellcolor{{blue!12}}{improvement_decls:.0f}}}\n'
        f'& \\multicolumn{{1}}{{rV{{2}}}}{{{original_derefs}}}\n'
        f'& \\multicolumn{{1}}{{rV{{2}}}}{{{translated_derefs}}}\n'
        f'& \\multicolumn{{1}}{{rV{{2}}}}{{\\cellcolor{{blue!12}}{improvement_derefs:.0f}}}\n'
        '\\bigstrut\\\\\n', end='')

print('\\hlineB{2}\n'
      '\\end{tabular}%\n'
      '}')

print('\\resizebox{\\linewidth}{!}{%\n'
      '\\begin{tabular}{lrrrrrrrrr}\n'
       '\\clineB{2-10}{2}')

print('''\\multicolumn{1}{lV{2}}{\\multirow{3}{*}{}} &
  \\multicolumn{3}{cV{2}}{Unsafe} &
  \\multicolumn{3}{cV{2}}{Unsafe} &
  \\multicolumn{3}{cV{2}}{Unsafe Call}\\bigstrut[t]\\\\
\\multicolumn{1}{lV{2}}{} &
  \\multicolumn{3}{cV{2}}{Lines ($\\nabla$)} & 
  \\multicolumn{3}{cV{2}}{Type Casts ($\\nabla$)} &
  \\multicolumn{3}{cV{2}}{Expressions  ($\\nabla$)} \\bigstrut[b]\\\\ \\clineB{2-10}{2}''')

print('''\\multicolumn{1}{lV{2}}{} &
  \\multicolumn{1}{rV{2}}{\\rotatebox{90}{Before~~}} &
  \\multicolumn{1}{rV{2}}{{\\rotatebox{90}{After}}} &
  \\multicolumn{1}{cV{2}}{$\\Delta\\%$} &
  \\multicolumn{1}{rV{2}}{\\rotatebox{90}{Before~~}} &
  \\multicolumn{1}{rV{2}}{{\\rotatebox{90}{After}}} &
  \\multicolumn{1}{cV{2}}{$\\Delta\\%$} &
  \\multicolumn{1}{rV{2}}{\\rotatebox{90}{Before~~}} &
  \\multicolumn{1}{rV{2}}{{\\rotatebox{90}{After}}} &
  \\multicolumn{1}{cV{2}}{$\\Delta\\%$} \\bigstrut\\\\ \\clineB{2-10}{2}
& \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} \\bigstrut\\\\[-1.33em] \\hlineB{2}''')

for program in data.keys():
    
    if 'original' not in data[program] or 'translated' not in data[program]:
      print(f"Skipping {program} — missing 'original' or 'translated'")
      continue
    
    original = data[program]['original']
    translated = data[program]['translated']

    original_unsafe = original['unsafe_spans']
    original_calls = original['unsafe_calls']
    original_casts = original['unsafe_casts']

    translated_unsafe = translated['unsafe_spans']
    translated_calls = translated['unsafe_calls']
    translated_casts = translated['unsafe_casts']

    improvement_unsafe = 100 * (original_unsafe - translated_unsafe) / original_unsafe
    improvement_calls = 100 * (original_calls - translated_calls) / original_calls
    improvement_casts = 100 * (original_casts - translated_casts) / original_casts

    print(f'\\multicolumn{{1}}{{V{{2}}lV{{2}}}}{{{program}}}\n'
        f'& \\multicolumn{{1}}{{rV{{2}}}}{{{original_unsafe}}}\n'
        f'& \\multicolumn{{1}}{{rV{{2}}}}{{{translated_unsafe}}}\n'
        f'& \\multicolumn{{1}}{{rV{{2}}}}{{\\cellcolor{{blue!12}}{improvement_unsafe:.0f}}}\n'
        f'& \\multicolumn{{1}}{{rV{{2}}}}{{{original_calls}}}\n'
        f'& \\multicolumn{{1}}{{rV{{2}}}}{{{translated_calls}}}\n'
        f'& \\multicolumn{{1}}{{rV{{2}}}}{{\\cellcolor{{blue!12}}{improvement_calls:.0f}}}\n'
        f'& \\multicolumn{{1}}{{rV{{2}}}}{{{original_casts}}}\n'
        f'& \\multicolumn{{1}}{{rV{{2}}}}{{{translated_casts}}}\n'
        f'& \\multicolumn{{1}}{{rV{{2}}}}{{\\cellcolor{{blue!12}}{improvement_casts:.0f}}}\n'
        '\\bigstrut\\\\\n', end='')

print('\\hlineB{2}\n'
      '\\end{tabular}%\n'
      '}')
