$ErrorActionPreference = "Stop"

# Docker image build
docker build --no-cache -t morphology-adaptive .

# Run container to generate trajectory and render GIF
# Note: matplotlib and imageio are installed via requirements.txt in the build step
docker run --rm -v ${PWD}:/morphology-adaptive -w /morphology-adaptive morphology-adaptive bash -c "python python/scripts/generate_trajectory_with_attn_policy.py --agent data/agents/biped --policy data/policies/attn --steps 50 --output trajectory_out && python python/scripts/render_trajectory.py --trajectory trajectory_out --output media/anim.gif"

echo "GIF generated at media/anim.gif"
