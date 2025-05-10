from pathlib import Path
import os
import subprocess
import re
import json

def run(command):
    try:
        result = subprocess.run(
            command,
            shell=True,
            timeout=25,
            stderr=subprocess.PIPE,
            stdout=subprocess.PIPE,
        )
        if result.returncode != 0:
            exec_output = result.stderr.decode('utf-8')
            if exec_output.strip() == '':
                exec_output = result.stdout.decode('utf-8')
            raise RuntimeError(exec_output)
    except subprocess.TimeoutExpired:
        raise RuntimeError("Timeout")
    
    # return the output of the command
    return result.stdout.decode('utf-8')

def parse_metrics_output(metrics_output):
    metrics = {}
    for line in metrics_output.split('\n'):
        
        # If the line is of the format "Unsafe spans: <num>", extract the num
        unsafe_spans = re.match(r'Unsafe lines: (\d+)', line)
        if unsafe_spans:
            if 'unsafe_spans' in metrics:
                metrics['unsafe_spans'] += int(unsafe_spans.group(1))
            else:
                metrics['unsafe_spans'] = int(unsafe_spans.group(1))

        unsafe_calls = re.match(r'Unsafe calls: (\d+)', line)
        if unsafe_calls:
            if 'unsafe_calls' in metrics:
                metrics['unsafe_calls'] += int(unsafe_calls.group(1))
            else:
                metrics['unsafe_calls'] = int(unsafe_calls.group(1))

        unsafe_casts = re.match(r'Unsafe lines: (\d+)', line)
        if unsafe_casts:
            if 'unsafe_casts' in metrics:
                metrics['unsafe_casts'] += int(unsafe_casts.group(1))
            else:
                metrics['unsafe_casts'] = int(unsafe_casts.group(1))
        
        # # If the line is of the format "Total spans: <num>", extract the num
        # total_spans = re.match(r'Total spans: (\d+)', line)
        # if total_spans:
        #     if 'total_spans' in metrics:
        #         metrics['total_spans'] += int(total_spans.group(1))
        #     else:
        #         metrics['total_spans'] = int(total_spans.group(1))
        
        # If the line is of the format "Raw pointer dereferences: <num>", extract the num
        raw_pointer_derefs = re.match(r'Raw pointer dereferences: (\d+)', line)
        if raw_pointer_derefs:
            if 'raw_pointer_derefs' in metrics:
                metrics['raw_pointer_derefs'] += int(raw_pointer_derefs.group(1))
            else:
                metrics['raw_pointer_derefs'] = int(raw_pointer_derefs.group(1))
        
        # If the line is of the format "Raw pointer declarations: <num>", extract the num
        raw_pointer_decls = re.match(r'Raw pointer declarations: (\d+)', line)
        if raw_pointer_decls:
            if 'raw_pointer_decls' in metrics:
                metrics['raw_pointer_decls'] += int(raw_pointer_decls.group(1))
            else:
                metrics['raw_pointer_decls'] = int(raw_pointer_decls.group(1))
        
    return metrics

if __name__ == "__main__":

    all_metrics = {}
    root_dir = Path('.').resolve()

    # Go to each directory in src/{}
    for dir in Path('src').glob('*'):
        if not dir.is_file():
            dir = dir.name
            print(f"Processing {dir}...")
            os.chdir(root_dir)
            subdir = Path('src/{}'.format(dir))
            original_subdir = Path(subdir, 'rust')
            translated_subdir = Path(subdir, 'rust_WIP')
            nochunk_subdir = Path(subdir, 'rust_WIP_nochunking')
            random_subdir = Path(subdir, 'rust_WIP_randomized')

            if not original_subdir.exists() or not translated_subdir.exists():
                continue
            os.chdir(root_dir)
            os.chdir(original_subdir)
            try:
                original_metrics = run('cargo metrics -- -Zselected-fns="../covered_funcs.txt"')
            except RuntimeError as e:
                print(f"Error in {original_subdir}")
                continue
            os.chdir(root_dir)
            os.chdir(translated_subdir)
            try:
                translated_metrics = run('cargo metrics -- -Zselected-fns="../covered_funcs.txt"')
            except RuntimeError as e:
                print(f"Error in {translated_subdir}")
                continue
            os.chdir(root_dir)

            all_metrics[dir] = {
                'original': parse_metrics_output(original_metrics),
                'translated': parse_metrics_output(translated_metrics)
            }
            
            if not nochunk_subdir.exists() or not random_subdir.exists():
                continue
            os.chdir(nochunk_subdir)
            try:
                nochunk_metrics = run('cargo metrics -- -Zselected-fns="../covered_funcs.txt"')
            except RuntimeError as e:
                print(f"Error in {nochunk_subdir}")
                continue
            os.chdir(root_dir)
            os.chdir(random_subdir)
            try:
                random_metrics = run('cargo metrics -- -Zselected-fns="../covered_funcs.txt"')
            except RuntimeError as e:
                print(f"Error in {random_subdir}")
                continue

            all_metrics[dir]['nochunking'] = parse_metrics_output(nochunk_metrics)
            all_metrics[dir]['randomized'] = parse_metrics_output(random_metrics)
    
    os.chdir(root_dir)
    # Write this dict to json
    with open('coreutils_metrics.json', 'w') as f:
        json.dump(all_metrics, f, indent=4)