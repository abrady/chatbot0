#!/usr/bin/env python3
"""
Setup script to discover Ollama models and generate a models.json file
for the candle chat to use when no model path is provided.
"""

import json
import subprocess
import sys
import re
from pathlib import Path

def run_command(cmd):
    """Run a command and return its output"""
    try:
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True, 
                              check=True, encoding='utf-8', errors='ignore')
        return result.stdout.strip()
    except subprocess.CalledProcessError as e:
        print(f"Error running command '{cmd}': {e}")
        return None
    except Exception as e:
        print(f"Unexpected error running '{cmd}': {e}")
        return None

def get_ollama_models():
    """Get list of available Ollama models"""
    output = run_command("ollama list")
    if not output:
        print("No Ollama models found or Ollama not available")
        return []
    
    models = []
    lines = output.split('\n')
    
    # Skip header line
    for line in lines[1:]:
        if line.strip():
            # Parse the model name (first column)
            parts = line.split()
            if parts:
                model_name = parts[0]
                models.append(model_name)
    
    return models

def get_model_path(model_name):
    """Get the file path for a specific model"""
    output = run_command(f'ollama show "{model_name}" --modelfile')
    if not output:
        return None
    
    # Look for the FROM line with a file path
    for line in output.split('\n'):
        if line.startswith('FROM ') and ('\\' in line or '/' in line):
            # Extract the path after FROM
            path = line[5:].strip()  # Remove 'FROM '
            return path
    
    return None

def main():
    print("* Discovering Ollama models...")
    
    models = get_ollama_models()
    if not models:
        print("X No models found. Make sure Ollama is installed and has models.")
        sys.exit(1)
    
    print(f"+ Found {len(models)} models:")
    for model in models:
        print(f"  - {model}")
    
    print("\n* Getting model paths...")
    
    model_data = []
    for model_name in models:
        print(f"  Processing {model_name}...")
        path = get_model_path(model_name)
        if path and Path(path).exists():
            # Get file size for display
            try:
                size = Path(path).stat().st_size
                size_mb = size / (1024 * 1024)
                size_str = f"{size_mb:.1f} MB" if size_mb < 1024 else f"{size_mb/1024:.1f} GB"
            except:
                size_str = "Unknown"
            
            model_data.append({
                "name": model_name,
                "path": path,
                "size": size_str
            })
        else:
            print(f"    ! Could not find file for {model_name}")
    
    if not model_data:
        print("X No valid model paths found.")
        sys.exit(1)
    
    # Write to models.json
    output_file = Path("models.json")
    with open(output_file, 'w') as f:
        json.dump({
            "models": model_data,
            "generated_by": "setup_models.py",
            "note": "This file is auto-generated. Run setup_models.py to refresh."
        }, f, indent=2)
    
    print(f"\n+ Generated {output_file} with {len(model_data)} models:")
    for model in model_data:
        print(f"  - {model['name']} ({model['size']})")
    
    print(f"\n> Now you can run 'cargo run candle-chat' without arguments")
    print("  and it will prompt you to select from these models!")

if __name__ == "__main__":
    main()