import argparse
import json
import os
import matplotlib.pyplot as plt
import matplotlib.tri as tri
import numpy as np
import imageio.v2 as imageio
from pathlib import Path

def render_frame(step_data, mesh_data, output_path):
    pos = np.array(step_data["pos1"]) # Use pos1 (end of step)
    triangles = np.array(mesh_data["triangles"])
    
    plt.figure(figsize=(8, 6))
    plt.gca().set_aspect('equal')
    
    # Plot mesh
    plt.triplot(pos[:, 0], pos[:, 1], triangles, 'g-', lw=1)
    plt.plot(pos[:, 0], pos[:, 1], 'go', ms=2)
    
    # Ground
    plt.axhline(0, color='black', lw=2)
    
    plt.grid(True)
    
    # Track center
    center = np.mean(pos, axis=0)
    plt.xlim(center[0] - 5, center[0] + 5)
    plt.ylim(-2, 8)
    
    plt.savefig(output_path)
    plt.close()

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--trajectory", type=str, required=True)
    parser.add_argument("--output", type=str, default="anim.gif")
    args = parser.parse_args()
    
    traj_dir = Path(args.trajectory)
    mesh_path = traj_dir / "mesh.json"
    steps_dir = traj_dir / "steps"
    
    if not mesh_path.exists():
        print(f"Error: {mesh_path} does not exist.")
        return

    with open(mesh_path) as f:
        mesh_data = json.load(f)
        
    step_files = sorted(steps_dir.glob("*.json"), key=lambda p: int(p.stem))
    
    images = []
    temp_dir = traj_dir / "frames"
    os.makedirs(temp_dir, exist_ok=True)
    
    print(f"Rendering {len(step_files)} frames...")
    
    for i, step_file in enumerate(step_files):
        with open(step_file) as f:
            step_data = json.load(f)
            
        frame_path = temp_dir / f"{i:04d}.png"
        render_frame(step_data, mesh_data, frame_path)
        images.append(imageio.imread(frame_path))
        
        if i % 10 == 0:
            print(f"Rendered frame {i}")
            
    # Save GIF
    imageio.mimsave(args.output, images, duration=0.1) # 10 fps (0.1s per frame)
    print(f"Saved GIF to {args.output}")

if __name__ == "__main__":
    main()
