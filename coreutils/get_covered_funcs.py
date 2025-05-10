from pathlib import Path
import os
import json

def safe_load_json(filepath):
    with open(filepath, "rb") as f:  # Open in binary mode
        raw_bytes = f.read()  # Read raw data

    json_str = raw_bytes.decode("latin-1", errors="backslashreplace").strip()

    if json_str.endswith(","):  
        json_str = json_str[:-1]  # Remove trailing comma
    if not json_str.startswith("["):
        json_str = "[" + json_str
    if not json_str.endswith("]"):
        json_str = json_str + "]"

    try:
        data = json.loads(json_str.strip(), strict=False)
        return data
    except:
        print(f"Failed to load JSON from {filepath}")
        return None
    
if __name__ == "__main__":
    
    instr_results = [Path("instrumentation", f) for f in os.listdir('instrumentation') if f.endswith('.json')]

    for result_file in instr_results:

        data = safe_load_json(result_file)
        if data is None:
            continue

        covered_funcs = set()
        for entry in data:
            covered_funcs.add(entry['name'])
        covered_funcs = list(covered_funcs)

        util_name = result_file.stem

        with open(f'src/{util_name}/covered_funcs.txt', 'w') as f:
            for func in covered_funcs:
                f.write(func + '\n')
        
        print("Wrote covered functions to src/{}/covered_funcs.txt".format(util_name))
    
