import torch
import json
import os
import sys

# Add current directory to path to import attn
sys.path.append(os.getcwd())

from attn.model import Model

def convert_model_to_json(model_dir, output_filename):
    model = Model.load(model_dir)
    state_dict = model.state_dict()
    
    weights = {}
    for key, value in state_dict.items():
        weights[key] = value.tolist()
        
    with open(output_filename, "w") as f:
        json.dump(weights, f)
    print(f"Converted {model_dir} to {output_filename}")

if __name__ == "__main__":
    model_dir = "data/policies/attn"
    output_filename = "public/data/policies/attn/weights.json"
    
    os.makedirs(os.path.dirname(output_filename), exist_ok=True)
    convert_model_to_json(model_dir, output_filename)
